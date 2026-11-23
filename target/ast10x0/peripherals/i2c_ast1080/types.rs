// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Core types for AST1080 I2C driver
//!
//! These types are portable and have no OS dependencies.

use ast1080_pac;

/// I2C controller identifier (0-13 for AST1080)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Controller(pub u8);

impl Controller {
    /// Create a new controller instance
    #[must_use]
    pub const fn new(id: u8) -> Option<Self> {
        if id < 14 { Some(Self(id)) } else { None }
    }

    /// Get the controller ID
    #[must_use]
    pub const fn id(&self) -> u8 {
        self.0
    }
}

// Implement conversions for compatibility
impl From<Controller> for u8 {
    fn from(c: Controller) -> u8 {
        c.0
    }
}

// NOTE: We intentionally don't implement From<u8> for Controller
// because it would bypass validation. Use Controller::new() or TryFrom instead.
impl TryFrom<u8> for Controller {
    type Error = ();

    fn try_from(id: u8) -> Result<Self, Self::Error> {
        Self::new(id).ok_or(())
    }
}

// ============================================================================
// Design Note: I2C Address Validation
// ============================================================================
//
// embedded-hal defines `SevenBitAddress` as a type alias for `u8`:
//   pub type SevenBitAddress = u8;
//
// This means I2C addresses are NOT validated at the type level in embedded-hal.
// We follow this convention for compatibility, but validate at runtime in:
// - SlaveConfig::new() - validates slave address is ≤ 0x7F
// - Master operations - hardware will NAK invalid addresses
//
// Alternative approaches considered but rejected:
// 1. Custom newtype - breaks embedded-hal I2c<SevenBitAddress> trait impl
// 2. Const generics - too restrictive, addresses often come from runtime config
//
// The tradeoff is: embedded-hal compatibility > compile-time address validation

/// Clock configuration for I2C timing calculations
///
/// This struct decouples the I2C driver from direct SCU/I2C global register access,
/// enabling:
/// - Unit testing without hardware
/// - Portability across different board configurations
/// - Clear ownership boundaries between system initialization and peripheral drivers
///
/// # Usage
///
/// For typical AST1080 configurations, use `ClockConfig::ast1080_default()`.
/// For custom configurations or testing, construct directly.
///
/// # Example
///
/// ```rust,no_run
/// use ast10x0_peripherals::i2c_ast1080::ClockConfig;
///
/// // Use default AST1080 configuration (50 MHz APB)
/// let clocks = ClockConfig::ast1080_default();
///
/// // Or read from hardware during system init
/// let clocks = ClockConfig::ast1080_default();
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ClockConfig {
    /// APB bus clock frequency in Hz
    pub apb_clock_hz: u32,
    /// Base clock 1 frequency in Hz (for Fast-plus mode, ~20 `MHz`)
    pub base_clk1_hz: u32,
    /// Base clock 2 frequency in Hz (for Fast mode, ~10 `MHz`)
    pub base_clk2_hz: u32,
    /// Base clock 3 frequency in Hz (for Standard mode, ~2.77 `MHz`)
    pub base_clk3_hz: u32,
    /// Base clock 4 frequency in Hz (for recovery timeout)
    pub base_clk4_hz: u32,
}

impl ClockConfig {
    /// Default AST1080 clock configuration
    ///
    /// Static configuration for boards configured with:
    /// - APB clock: 50 `MHz`
    /// - I2CG10 register: `0x6222_0803`
    ///
    /// This can be used when the actual hardware configuration matches
    /// these defaults, or for testing purposes.
    #[must_use]
    pub const fn ast1080_default() -> Self {
        // Requires a board-supplied 50 MHz APB clock.
        // I2CG10 = 0x6222_0803:
        //   base_clk1 divisor = 0x03 → 50M / ((3+2)/2) = 20 MHz
        //   base_clk2 divisor = 0x08 → 50M / ((8+2)/2) = 10 MHz
        //   base_clk3 divisor = 0x22 → 50M / ((34+2)/2) = 2.77 MHz
        //   base_clk4 divisor = 0x62 → 50M / ((98+2)/2) = 1 MHz
        Self {
            apb_clock_hz: 50_000_000,
            base_clk1_hz: 20_000_000,
            base_clk2_hz: 10_000_000,
            base_clk3_hz: 2_770_000,
            base_clk4_hz: 1_000_000,
        }
    }

    /// Read clock configuration from hardware registers
    ///
    /// Reads I2C base-clock dividers using the APB frequency supplied by
    /// the board. No AST1060 SCU clock assumptions are made.
    ///
    /// # Safety
    ///
    /// This function accesses I2C global registers. It should be
    /// called during system initialization when these peripherals are
    /// properly configured.
    #[must_use]
    pub fn from_hardware(apb_clock_hz: u32) -> Self {
        // The board supplies the APB clock; AST1080 SCU differs from AST1060.
        // Read base clock dividers from I2C global registers
        let i2cg = unsafe { &*ast1080_pac::I2cglobal::ptr() };
        let i2cglobal010 = i2cg.i2cglobal010().read();

        // Formula: base_clk = APB * 10 / ((divisor + 2) * 10 / 2)
        let calc_base = |divisor: u8| -> u32 {
            let divisor_u32 = u32::from(divisor);
            (u64::from(apb_clock_hz) * 2 / u64::from(divisor_u32 + 2)) as u32
        };

        Self {
            apb_clock_hz,
            base_clk1_hz: calc_base(i2cglobal010.basediv1().bits()),
            base_clk2_hz: calc_base(i2cglobal010.basediv2().bits()),
            base_clk3_hz: calc_base(i2cglobal010.basediv3().bits()),
            base_clk4_hz: calc_base(i2cglobal010.basediv4().bits()),
        }
    }
}

impl Default for ClockConfig {
    fn default() -> Self {
        Self::ast1080_default()
    }
}

/// I2C configuration
///
/// Default configuration is optimized for MCTP-over-I2C:
/// - Fast mode (400 kHz) - standard MCTP speed
/// - Buffer mode - efficient for MCTP packet transfers
/// - `SMBus` timeout enabled - required for robust MCTP operation
/// - Static 50 MHz APB configuration; override for the actual board clocks
#[derive(Debug, Clone, Copy)]
pub struct I2cConfig {
    /// Transfer mode (byte-by-byte or buffer)
    pub xfer_mode: I2cXferMode,
    /// Bus speed (Standard/Fast/FastPlus)
    pub speed: I2cSpeed,
    /// Enable multi-master support
    pub multi_master: bool,
    /// Enable `SMBus` timeout detection (25-35ms per `SMBus` spec)
    pub smbus_timeout: bool,
    /// Enable `SMBus` alert interrupt
    pub smbus_alert: bool,
    /// Clock configuration for timing calculations
    pub clock_config: ClockConfig,
}

impl Default for I2cConfig {
    /// Create default I2C configuration with static 50 MHz APB clocks
    ///
    /// The board must configure a 50 MHz APB clock and initialize I2C global
    /// dividers before using these defaults. For other board clocks, set
    /// `clock_config` using `ClockConfig::from_hardware(apb_clock_hz)`.
    ///
    /// For DMA mode, set `xfer_mode` to [`I2cXferMode::DmaMode`] and
    /// pass a DMA buffer via
    /// [`Ast1080I2c::new_with_dma`](super::controller::Ast1080I2c::new_with_dma).
    fn default() -> Self {
        Self {
            xfer_mode: I2cXferMode::BufferMode,
            speed: I2cSpeed::Fast,
            multi_master: false,
            smbus_timeout: true,
            smbus_alert: false,
            clock_config: ClockConfig::ast1080_default(),
        }
    }
}

impl I2cConfig {
    /// Create configuration with static default clocks (for testing)
    ///
    /// Uses the same static clock values as `default()`.
    #[must_use]
    pub fn with_static_clocks() -> Self {
        Self {
            clock_config: ClockConfig::ast1080_default(),
            ..Self {
                xfer_mode: I2cXferMode::BufferMode,
                speed: I2cSpeed::Fast,
                multi_master: false,
                smbus_timeout: true,
                smbus_alert: false,
                clock_config: ClockConfig::ast1080_default(),
            }
        }
    }
}

/// Transfer mode selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cXferMode {
    /// Byte-by-byte mode (1 byte at a time)
    ByteMode,
    /// Buffer mode (up to 32 bytes via hardware buffer)
    BufferMode,
    /// DMA mode (up to 4096 bytes via system memory DMA)
    ///
    /// Requires a pre-allocated, cache-coherent DMA buffer passed to
    /// [`Ast1080I2c::new_with_dma`]. The buffer must be placed in a
    /// non-cached memory region (e.g. `#[link_section = ".ram_nc"]`).
    DmaMode,
}

/// I2C bus speed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum I2cSpeed {
    /// Standard mode: 100 kHz
    Standard,
    /// Fast mode: 400 kHz
    Fast,
    /// Fast-plus mode: 1 `MHz`
    FastPlus,
}
