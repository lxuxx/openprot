// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! AST1080 timing configuration.
//!
//! I2C04 contains an eight-bit divider value, not a base-clock index.
//! The source frequency is APB / (divider + 1). Prefer the first three
//! programmed global dividers, then search all 256 divider values.
//! Global initialization uses 0xC622_0904; the board supplies APB frequency.
//! SMBus timeout counts live in I2C74, separate from function control.

use super::error::I2cError;
use super::types::{I2cConfig, I2cSpeed};
use ast1080_pac::i2c::RegisterBlock;
use core::cmp::min;

/// I2C speed frequencies in Hz
const SPEED_STANDARD: u32 = 100_000;
const SPEED_FAST: u32 = 400_000;
const SPEED_FAST_PLUS: u32 = 1_000_000;

/// Configure I2C timing based on speed and injected clock configuration
///
/// Uses the `ClockConfig` from `I2cConfig` to calculate proper SCL timing
/// using the supplied APB frequency and current global divider register.
///
/// # Arguments
///
/// * `regs` - Reference to the I2C controller register block
/// * `config` - I2C configuration containing speed, timeout, and clock settings
///
/// # Returns
///
/// * `Ok(())` on successful configuration
/// * `Err(I2cError::Invalid)` if timing cannot be achieved
pub fn configure_timing(regs: &RegisterBlock, config: &I2cConfig) -> Result<(), I2cError> {
    let clocks = &config.clock_config;

    // Step 1: Get target speed
    let target_speed = match config.speed {
        I2cSpeed::Standard => SPEED_STANDARD,
        I2cSpeed::Fast => SPEED_FAST,
        I2cSpeed::FastPlus => SPEED_FAST_PLUS,
    };

    // Step 2: Find best base clock and divider using injected clock config
    let global = unsafe { &*ast1080_pac::I2cglobal::ptr() };
    let (baseclk_idx, divisor) = calculate_baseclk(
        clocks.apb_clock_hz,
        global.i2cglobal010().read().bits(),
        target_speed,
    )?;

    // Step 3: Calculate SCL high/low times
    // I2C spec: tLOW should be slightly longer than tHIGH
    // Default ratio: 9/16 low, 7/16 high (asymmetric for spec compliance)
    let scl_low = calculate_scl_low(divisor);
    let scl_high = calculate_scl_high(divisor, scl_low);
    let scl_high_min = scl_high.saturating_sub(1);

    // AST1080 uses separate master/slave counts at I2C74. Do not write
    // these to I2C00 (function control), despite the Zephyr reference typo.
    // Retain count 8 for the boolean timeout option; wall-clock duration
    // must be validated against the actual board clocks.
    let (timeout_divisor, timeout_count) = if config.smbus_timeout {
        (3u8, 1u8)
    } else {
        (0u8, 0u8)
    };
    regs.i2c74().modify(|_, w| unsafe {
        w.mtimeout()
            .bits(timeout_count)
            .stimeout()
            .bits(timeout_count)
    });

    // Step 5: Write AC timing register with all fields in a single write
    // This avoids corrupting fields with multiple separate writes
    regs.i2c04().write(|w| unsafe {
        w.actimebase()
            .bits((baseclk_idx & 0xff) as u8)
            .tcklow()
            .bits(scl_low)
            .tckhigh()
            .bits(scl_high)
            .tckhighmin()
            .bits(scl_high_min)
            .timeoutbase()
            .bits(timeout_divisor)
    });

    Ok(())
}

/// Select a representable period without exceeding the requested bus speed.
/// A zero divider is valid. Periods below three cycles cannot encode both
/// high time and high-minimum time; periods above 32 do not fit.
fn calculate_baseclk(apb_hz: u32, dividers: u32, target_hz: u32) -> Result<(u32, u32), I2cError> {
    if apb_hz == 0 || target_hz == 0 {
        return Err(I2cError::Invalid);
    }
    let candidate = |divider: u32| -> Option<(u32, u32)> {
        let denominator = u64::from(divider + 1) * u64::from(target_hz);
        let period = u64::from(apb_hz).div_ceil(denominator);
        if !(3..=32).contains(&period) {
            return None;
        }
        Some((divider, period as u32))
    };
    for shift in [0, 8, 16] {
        if let Some(result) = candidate((dividers >> shift) & 0xff) {
            return Ok(result);
        }
    }
    for divider in 0..=255 {
        if let Some(result) = candidate(divider) {
            return Ok(result);
        }
    }
    Err(I2cError::Invalid)
}

/// Calculate SCL low time cycles
///
/// I2C specification requires tLOW to be longer than tHIGH.
/// We use 9/16 of the period for low time (56.25%).
fn calculate_scl_low(divisor: u32) -> u8 {
    // scl_low = (ratio * 9/16) - 1, clamped to 4 bits
    let scl_low = (divisor * 9 / 16).saturating_sub(1);
    #[allow(clippy::cast_possible_truncation)]
    let result = min(scl_low, 0xf) as u8;
    result
}

/// Calculate SCL high time cycles
///
/// High time is the remainder after low time and 2 cycles for edges.
fn calculate_scl_high(divisor: u32, scl_low: u8) -> u8 {
    // scl_high = ratio - scl_low - 2 (2 cycles for rise/fall edges)
    let scl_high = divisor.saturating_sub(u32::from(scl_low)).saturating_sub(2);
    #[allow(clippy::cast_possible_truncation)]
    let result = min(scl_high, 0xf) as u8;
    result
}
