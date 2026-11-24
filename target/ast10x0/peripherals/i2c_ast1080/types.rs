// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Core types for AST1080 I2C driver
//!
//! These types are portable and have no OS dependencies.

use super::global::DEFAULT_I2C_CLOCK_DIVIDERS;
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
/// This struct supplies the APB frequency and a snapshot of the base clocks,
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
/// let clocks = ClockConfig::from_hardware(50_000_000);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ClockConfig {
    /// APB bus clock frequency in Hz
    pub apb_clock_hz: u32,
    /// Base clock 1 frequency in Hz
    pub base_clk1_hz: u32,
    /// Base clock 2 frequency in Hz
    pub base_clk2_hz: u32,
    /// Base clock 3 frequency in Hz
    pub base_clk3_hz: u32,
    /// Base clock 4 frequency in Hz (for recovery timeout)
    pub base_clk4_hz: u32,
}

impl ClockConfig {
    /// Static clocks for a board with 50 MHz APB and the default global dividers.
    #[must_use]
    pub const fn ast1080_default() -> Self {
        Self::from_dividers(50_000_000, DEFAULT_I2C_CLOCK_DIVIDERS)
    }

    /// Decode AST1080 base clocks: APB / (divider + 1).
    #[must_use]
    pub const fn from_dividers(apb_clock_hz: u32, dividers: u32) -> Self {
        Self {
            apb_clock_hz,
            base_clk1_hz: apb_clock_hz / ((dividers & 0xff) + 1),
            base_clk2_hz: apb_clock_hz / (((dividers >> 8) & 0xff) + 1),
            base_clk3_hz: apb_clock_hz / (((dividers >> 16) & 0xff) + 1),
            base_clk4_hz: apb_clock_hz / (((dividers >> 24) & 0xff) + 1),
        }
    }

    /// Read the global dividers using the board-supplied APB frequency.
    ///
    /// Call during initialization with the I2C clock enabled and global
    /// dividers configured. This does not read or configure SCU clocks.
    #[must_use]
    pub fn from_hardware(apb_clock_hz: u32) -> Self {
        let i2cg = unsafe { &*ast1080_pac::I2cglobal::ptr() };
        Self::from_dividers(apb_clock_hz, i2cg.i2cglobal010().read().bits())
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
    /// Enable master/slave timeout counters (count 8, timeout base selector 3).
    /// The resulting duration depends on the board clock configuration.
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
