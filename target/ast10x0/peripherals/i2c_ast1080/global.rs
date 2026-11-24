// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! I2C global hardware initialization utility
//!
//! Configures I2C global registers (i2cglobal00c, i2cglobal010).
//! This must be called once after SCU has been configured by the board layer.
//!
//! # Initialization Order
//!
//! The board layer must call init functions in this order:
//! 1. Enable I2C clock via SCU
//! 2. Assert I2C reset via SCU
//! 3. Delay
//! 4. Deassert I2C reset via SCU
//! 5. Delay
//! 6. Call `init_i2c_global()` to configure I2C registers
//!
//! # Register Details
//!
//! ## I2CG0C - I2C Global Control Register
//! - `clkdivsel`: Enable new clock divider mode
//! - `newregmode`: Read-only on AST1080; no mode-selection write is needed
//! - `srxmode`: RX buffer empty action
//!
//! ## I2CG10 - I2C Global Clock Divider Register
//! The default is 0xC622_0904. Each base clock is APB / (divider + 1).
//! At 50 MHz APB the four clocks are 10 MHz, 5 MHz, approximately
//! 1.429 MHz, and approximately 251.256 kHz. Board clock setup must match.

use ast1080_pac;

/// Shared default for global initialization and static clock configuration.
pub const DEFAULT_I2C_CLOCK_DIVIDERS: u32 = 0xC622_0904;

/// Configure I2C global registers (one-time init, after SCU setup).
///
/// # Safety
/// - Must be called only once, after SCU has enabled I2C clock and deasserted reset.
/// - Not thread-safe; caller must ensure single invocation.
/// - Assumes I2C controller clock is already enabled by the board layer.
pub unsafe fn init_i2c_global() {
    let i2cg = unsafe { &*ast1080_pac::I2cglobal::ptr() };

    // Configure global settings
    i2cg.i2cglobal00c()
        .write(|w| w.clkdivsel().set_bit().srxmode().set_bit());

    // Keep static ClockConfig defaults synchronized with this register value.
    unsafe {
        i2cg.i2cglobal010()
            .write(|w| w.bits(DEFAULT_I2C_CLOCK_DIVIDERS))
    };
}
