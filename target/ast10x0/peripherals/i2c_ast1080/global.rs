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
//! Value: `0x6222_0803`
//! - Bits [31:24] = 0x62: Base clk4 for auto recovery timeout
//! - Bits [23:16] = 0x22: Base clk3 for Standard-mode (100kHz), tBuf=5.76us
//! - Bits [15:8]  = 0x08: Base clk2 for Fast-mode (400kHz), tBuf=1.6us
//! - Bits [7:0]   = 0x03: Base clk1 for Fast-mode Plus (1MHz), tBuf=0.8us
//!
//! Based on APB clock = 50MHz:
//! ```text
//! div  : scl       : baseclk [APB/((div/2) + 1)] : tBuf [1/bclk * 16]
//! 0x03 : 1MHz      : 20MHz                       : 0.8us  (Fast-mode Plus)
//! 0x08 : 400kHz    : 10MHz                       : 1.6us  (Fast-mode)
//! 0x22 : 99.21kHz  : 2.77MHz                     : 5.76us (Standard-mode)
//! ```

use ast1080_pac;

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

    // Set base clock dividers for different speeds
    // APB clk: 50MHz
    // I2CG10[31:24] = 0xC6: base clk4 for i2c auto recovery timeout counter
    // I2CG10[23:16] = 0x22: base clk3 for Standard-mode (100kHz) min tBuf 4.7us
    // I2CG10[15:8]  = 0x09: base clk2 for Fast-mode (400kHz) min tBuf 1.3us
    // I2CG10[7:0]   = 0x04: base clk1 for Fast-mode Plus (1MHz) min tBuf 0.5us
    unsafe { i2cg.i2cglobal010().write(|w| w.bits(0xC622_0904)) };
}
