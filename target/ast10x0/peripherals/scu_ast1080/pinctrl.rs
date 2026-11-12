// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! SCU Pin Control (Pinctrl) for multi-function pin configuration.
//!
//! Provides register-level access to configure AST1060 pins for different
//! functions (I2C, I3C, GPIO, etc.) via SCU multiplex registers.

use super::ScuRegisters;
use paste::paste;

/// Describes a single pin configuration operation.
#[derive(Clone, Copy, Debug)]
pub struct PinctrlPin {
    /// SCU register offset (0x410, 0x414, 0x690, etc.)
    pub offset: u32,
    /// Bit position within the register (0-31)
    pub bit: u32,
    /// true = clear bit, false = set bit
    pub clear: bool,
}

/// Macro to generate paired pin constants for set/clear operations.
macro_rules! gen_pin_pairs {
    ($reg_name:ident, $offset:expr, $bit:expr) => {
        paste! {
            pub const [<PIN_ $reg_name _ $bit>]: PinctrlPin = PinctrlPin {
                offset: $offset,
                bit: $bit,
                clear: false,
            };

            pub const [<CLR_PIN_ $reg_name _ $bit>]: PinctrlPin = PinctrlPin {
                offset: $offset,
                bit: $bit,
                clear: true,
            };
        }
    };
}

// Generate individual pin constants for each SCU register and bit position
paste! {
    gen_pin_pairs!(SCU400, 0x400, 0);
    gen_pin_pairs!(SCU400, 0x400, 1);
    gen_pin_pairs!(SCU400, 0x400, 2);
    gen_pin_pairs!(SCU400, 0x400, 3);
    gen_pin_pairs!(SCU400, 0x400, 4);
    gen_pin_pairs!(SCU400, 0x400, 5);
    gen_pin_pairs!(SCU400, 0x400, 6);
    gen_pin_pairs!(SCU400, 0x400, 7);
    gen_pin_pairs!(SCU400, 0x400, 8);
    gen_pin_pairs!(SCU400, 0x400, 9);
    gen_pin_pairs!(SCU400, 0x400, 10);
    gen_pin_pairs!(SCU400, 0x400, 11);
    gen_pin_pairs!(SCU400, 0x400, 12);
    gen_pin_pairs!(SCU400, 0x400, 13);
    gen_pin_pairs!(SCU400, 0x400, 14);
    gen_pin_pairs!(SCU400, 0x400, 15);
    gen_pin_pairs!(SCU400, 0x400, 16);
    gen_pin_pairs!(SCU400, 0x400, 17);
    gen_pin_pairs!(SCU400, 0x400, 18);
    gen_pin_pairs!(SCU400, 0x400, 19);
    gen_pin_pairs!(SCU400, 0x400, 20);
    gen_pin_pairs!(SCU400, 0x400, 21);
    gen_pin_pairs!(SCU400, 0x400, 22);
    gen_pin_pairs!(SCU400, 0x400, 23);
    gen_pin_pairs!(SCU400, 0x400, 24);
    gen_pin_pairs!(SCU400, 0x400, 25);
    gen_pin_pairs!(SCU400, 0x400, 26);
    gen_pin_pairs!(SCU400, 0x400, 27);
    gen_pin_pairs!(SCU400, 0x400, 28);
    gen_pin_pairs!(SCU400, 0x400, 29);
    gen_pin_pairs!(SCU400, 0x400, 30);
    gen_pin_pairs!(SCU400, 0x400, 31);

    gen_pin_pairs!(SCU404, 0x404, 0);
    gen_pin_pairs!(SCU404, 0x404, 1);
    gen_pin_pairs!(SCU404, 0x404, 2);
    gen_pin_pairs!(SCU404, 0x404, 3);
    gen_pin_pairs!(SCU404, 0x404, 4);
    gen_pin_pairs!(SCU404, 0x404, 5);
    gen_pin_pairs!(SCU404, 0x404, 6);
    gen_pin_pairs!(SCU404, 0x404, 7);
    gen_pin_pairs!(SCU404, 0x404, 8);
    gen_pin_pairs!(SCU404, 0x404, 9);
    gen_pin_pairs!(SCU404, 0x404, 10);
    gen_pin_pairs!(SCU404, 0x404, 11);
    gen_pin_pairs!(SCU404, 0x404, 12);
    gen_pin_pairs!(SCU404, 0x404, 13);
    gen_pin_pairs!(SCU404, 0x404, 14);
    gen_pin_pairs!(SCU404, 0x404, 15);
    gen_pin_pairs!(SCU404, 0x404, 16);
    gen_pin_pairs!(SCU404, 0x404, 17);
    gen_pin_pairs!(SCU404, 0x404, 18);
    gen_pin_pairs!(SCU404, 0x404, 19);
    gen_pin_pairs!(SCU404, 0x404, 20);
    gen_pin_pairs!(SCU404, 0x404, 21);
    gen_pin_pairs!(SCU404, 0x404, 22);
    gen_pin_pairs!(SCU404, 0x404, 23);
    gen_pin_pairs!(SCU404, 0x404, 24);
    gen_pin_pairs!(SCU404, 0x404, 25);
    gen_pin_pairs!(SCU404, 0x404, 26);
    gen_pin_pairs!(SCU404, 0x404, 27);
    gen_pin_pairs!(SCU404, 0x404, 28);
    gen_pin_pairs!(SCU404, 0x404, 29);
    gen_pin_pairs!(SCU404, 0x404, 30);
    gen_pin_pairs!(SCU404, 0x404, 31);

    gen_pin_pairs!(SCU408, 0x408, 0);
    gen_pin_pairs!(SCU408, 0x408, 1);
    gen_pin_pairs!(SCU408, 0x408, 2);
    gen_pin_pairs!(SCU408, 0x408, 3);
    gen_pin_pairs!(SCU408, 0x408, 4);
    gen_pin_pairs!(SCU408, 0x408, 5);
    gen_pin_pairs!(SCU408, 0x408, 6);
    gen_pin_pairs!(SCU408, 0x408, 7);
    gen_pin_pairs!(SCU408, 0x408, 8);
    gen_pin_pairs!(SCU408, 0x408, 9);
    gen_pin_pairs!(SCU408, 0x408, 10);
    gen_pin_pairs!(SCU408, 0x408, 11);
    gen_pin_pairs!(SCU408, 0x408, 12);
    gen_pin_pairs!(SCU408, 0x408, 13);
    gen_pin_pairs!(SCU408, 0x408, 14);
    gen_pin_pairs!(SCU408, 0x408, 15);
    gen_pin_pairs!(SCU408, 0x408, 16);
    gen_pin_pairs!(SCU408, 0x408, 17);
    gen_pin_pairs!(SCU408, 0x408, 18);
    gen_pin_pairs!(SCU408, 0x408, 19);
    gen_pin_pairs!(SCU408, 0x408, 20);
    gen_pin_pairs!(SCU408, 0x408, 21);
    gen_pin_pairs!(SCU408, 0x408, 22);
    gen_pin_pairs!(SCU408, 0x408, 23);
    gen_pin_pairs!(SCU408, 0x408, 24);
    gen_pin_pairs!(SCU408, 0x408, 25);
    gen_pin_pairs!(SCU408, 0x408, 26);
    gen_pin_pairs!(SCU408, 0x408, 27);
    gen_pin_pairs!(SCU408, 0x408, 28);
    gen_pin_pairs!(SCU408, 0x408, 29);
    gen_pin_pairs!(SCU408, 0x408, 30);
    gen_pin_pairs!(SCU408, 0x408, 31);

    gen_pin_pairs!(SCU410, 0x410, 0);
    gen_pin_pairs!(SCU410, 0x410, 1);
    gen_pin_pairs!(SCU410, 0x410, 2);
    gen_pin_pairs!(SCU410, 0x410, 3);
    gen_pin_pairs!(SCU410, 0x410, 4);
    gen_pin_pairs!(SCU410, 0x410, 5);
    gen_pin_pairs!(SCU410, 0x410, 6);
    gen_pin_pairs!(SCU410, 0x410, 7);
    gen_pin_pairs!(SCU410, 0x410, 8);
    gen_pin_pairs!(SCU410, 0x410, 9);
    gen_pin_pairs!(SCU410, 0x410, 10);
    gen_pin_pairs!(SCU410, 0x410, 11);
    gen_pin_pairs!(SCU410, 0x410, 12);
    gen_pin_pairs!(SCU410, 0x410, 13);
    gen_pin_pairs!(SCU410, 0x410, 14);
    gen_pin_pairs!(SCU410, 0x410, 15);
    gen_pin_pairs!(SCU410, 0x410, 16);
    gen_pin_pairs!(SCU410, 0x410, 17);
    gen_pin_pairs!(SCU410, 0x410, 18);
    gen_pin_pairs!(SCU410, 0x410, 19);
    gen_pin_pairs!(SCU410, 0x410, 20);
    gen_pin_pairs!(SCU410, 0x410, 21);
    gen_pin_pairs!(SCU410, 0x410, 22);
    gen_pin_pairs!(SCU410, 0x410, 23);
    gen_pin_pairs!(SCU410, 0x410, 24);
    gen_pin_pairs!(SCU410, 0x410, 25);
    gen_pin_pairs!(SCU410, 0x410, 26);
    gen_pin_pairs!(SCU410, 0x410, 27);
    gen_pin_pairs!(SCU410, 0x410, 28);
    gen_pin_pairs!(SCU410, 0x410, 29);
    gen_pin_pairs!(SCU410, 0x410, 30);
    gen_pin_pairs!(SCU410, 0x410, 31);

    gen_pin_pairs!(SCU414, 0x414, 0);
    gen_pin_pairs!(SCU414, 0x414, 1);
    gen_pin_pairs!(SCU414, 0x414, 2);
    gen_pin_pairs!(SCU414, 0x414, 3);
    gen_pin_pairs!(SCU414, 0x414, 4);
    gen_pin_pairs!(SCU414, 0x414, 5);
    gen_pin_pairs!(SCU414, 0x414, 6);
    gen_pin_pairs!(SCU414, 0x414, 7);
    gen_pin_pairs!(SCU414, 0x414, 8);
    gen_pin_pairs!(SCU414, 0x414, 9);
    gen_pin_pairs!(SCU414, 0x414, 10);
    gen_pin_pairs!(SCU414, 0x414, 11);
    gen_pin_pairs!(SCU414, 0x414, 12);
    gen_pin_pairs!(SCU414, 0x414, 13);
    gen_pin_pairs!(SCU414, 0x414, 14);
    gen_pin_pairs!(SCU414, 0x414, 15);
    gen_pin_pairs!(SCU414, 0x414, 16);
    gen_pin_pairs!(SCU414, 0x414, 17);
    gen_pin_pairs!(SCU414, 0x414, 18);
    gen_pin_pairs!(SCU414, 0x414, 19);
    gen_pin_pairs!(SCU414, 0x414, 20);
    gen_pin_pairs!(SCU414, 0x414, 21);
    gen_pin_pairs!(SCU414, 0x414, 22);
    gen_pin_pairs!(SCU414, 0x414, 23);
    gen_pin_pairs!(SCU414, 0x414, 24);
    gen_pin_pairs!(SCU414, 0x414, 25);
    gen_pin_pairs!(SCU414, 0x414, 26);
    gen_pin_pairs!(SCU414, 0x414, 27);
    gen_pin_pairs!(SCU414, 0x414, 28);
    gen_pin_pairs!(SCU414, 0x414, 29);
    gen_pin_pairs!(SCU414, 0x414, 30);
    gen_pin_pairs!(SCU414, 0x414, 31);

    gen_pin_pairs!(SCU418, 0x418, 0);
    gen_pin_pairs!(SCU418, 0x418, 1);
    gen_pin_pairs!(SCU418, 0x418, 2);
    gen_pin_pairs!(SCU418, 0x418, 3);
    gen_pin_pairs!(SCU418, 0x418, 4);
    gen_pin_pairs!(SCU418, 0x418, 5);
    gen_pin_pairs!(SCU418, 0x418, 6);
    gen_pin_pairs!(SCU418, 0x418, 7);
    gen_pin_pairs!(SCU418, 0x418, 8);
    gen_pin_pairs!(SCU418, 0x418, 9);
    gen_pin_pairs!(SCU418, 0x418, 10);
    gen_pin_pairs!(SCU418, 0x418, 11);
    gen_pin_pairs!(SCU418, 0x418, 12);
    gen_pin_pairs!(SCU418, 0x418, 13);
    gen_pin_pairs!(SCU418, 0x418, 14);
    gen_pin_pairs!(SCU418, 0x418, 15);
    gen_pin_pairs!(SCU418, 0x418, 16);
    gen_pin_pairs!(SCU418, 0x418, 17);
    gen_pin_pairs!(SCU418, 0x418, 18);
    gen_pin_pairs!(SCU418, 0x418, 19);
    gen_pin_pairs!(SCU418, 0x418, 20);
    gen_pin_pairs!(SCU418, 0x418, 21);
    gen_pin_pairs!(SCU418, 0x418, 22);
    gen_pin_pairs!(SCU418, 0x418, 23);
    gen_pin_pairs!(SCU418, 0x418, 24);
    gen_pin_pairs!(SCU418, 0x418, 25);
    gen_pin_pairs!(SCU418, 0x418, 26);
    gen_pin_pairs!(SCU418, 0x418, 27);
    gen_pin_pairs!(SCU418, 0x418, 28);
    gen_pin_pairs!(SCU418, 0x418, 29);
    gen_pin_pairs!(SCU418, 0x418, 30);
    gen_pin_pairs!(SCU418, 0x418, 31);

    gen_pin_pairs!(SCU41C, 0x41C, 0);
    gen_pin_pairs!(SCU41C, 0x41C, 1);
    gen_pin_pairs!(SCU41C, 0x41C, 2);
    gen_pin_pairs!(SCU41C, 0x41C, 3);
    gen_pin_pairs!(SCU41C, 0x41C, 4);
    gen_pin_pairs!(SCU41C, 0x41C, 5);
    gen_pin_pairs!(SCU41C, 0x41C, 6);
    gen_pin_pairs!(SCU41C, 0x41C, 7);
    gen_pin_pairs!(SCU41C, 0x41C, 8);
    gen_pin_pairs!(SCU41C, 0x41C, 9);
    gen_pin_pairs!(SCU41C, 0x41C, 10);
    gen_pin_pairs!(SCU41C, 0x41C, 11);
    gen_pin_pairs!(SCU41C, 0x41C, 12);
    gen_pin_pairs!(SCU41C, 0x41C, 13);
    gen_pin_pairs!(SCU41C, 0x41C, 14);
    gen_pin_pairs!(SCU41C, 0x41C, 15);
    gen_pin_pairs!(SCU41C, 0x41C, 16);
    gen_pin_pairs!(SCU41C, 0x41C, 17);
    gen_pin_pairs!(SCU41C, 0x41C, 18);
    gen_pin_pairs!(SCU41C, 0x41C, 19);
    gen_pin_pairs!(SCU41C, 0x41C, 20);
    gen_pin_pairs!(SCU41C, 0x41C, 21);
    gen_pin_pairs!(SCU41C, 0x41C, 22);
    gen_pin_pairs!(SCU41C, 0x41C, 23);
    gen_pin_pairs!(SCU41C, 0x41C, 24);
    gen_pin_pairs!(SCU41C, 0x41C, 25);
    gen_pin_pairs!(SCU41C, 0x41C, 26);
    gen_pin_pairs!(SCU41C, 0x41C, 27);
    gen_pin_pairs!(SCU41C, 0x41C, 28);
    gen_pin_pairs!(SCU41C, 0x41C, 29);
    gen_pin_pairs!(SCU41C, 0x41C, 30);
    gen_pin_pairs!(SCU41C, 0x41C, 31);

    gen_pin_pairs!(SCU430, 0x430, 0);
    gen_pin_pairs!(SCU430, 0x430, 1);
    gen_pin_pairs!(SCU430, 0x430, 2);
    gen_pin_pairs!(SCU430, 0x430, 3);
    gen_pin_pairs!(SCU430, 0x430, 4);
    gen_pin_pairs!(SCU430, 0x430, 5);
    gen_pin_pairs!(SCU430, 0x430, 6);
    gen_pin_pairs!(SCU430, 0x430, 7);
    gen_pin_pairs!(SCU430, 0x430, 8);
    gen_pin_pairs!(SCU430, 0x430, 9);
    gen_pin_pairs!(SCU430, 0x430, 10);
    gen_pin_pairs!(SCU430, 0x430, 11);
    gen_pin_pairs!(SCU430, 0x430, 12);
    gen_pin_pairs!(SCU430, 0x430, 13);
    gen_pin_pairs!(SCU430, 0x430, 14);
    gen_pin_pairs!(SCU430, 0x430, 15);
    gen_pin_pairs!(SCU430, 0x430, 16);
    gen_pin_pairs!(SCU430, 0x430, 17);
    gen_pin_pairs!(SCU430, 0x430, 18);
    gen_pin_pairs!(SCU430, 0x430, 19);
    gen_pin_pairs!(SCU430, 0x430, 20);
    gen_pin_pairs!(SCU430, 0x430, 21);
    gen_pin_pairs!(SCU430, 0x430, 22);
    gen_pin_pairs!(SCU430, 0x430, 23);
    gen_pin_pairs!(SCU430, 0x430, 24);
    gen_pin_pairs!(SCU430, 0x430, 25);
    gen_pin_pairs!(SCU430, 0x430, 26);
    gen_pin_pairs!(SCU430, 0x430, 27);
    gen_pin_pairs!(SCU430, 0x430, 28);
    gen_pin_pairs!(SCU430, 0x430, 29);
    gen_pin_pairs!(SCU430, 0x430, 30);
    gen_pin_pairs!(SCU430, 0x430, 31);

    gen_pin_pairs!(SCU434, 0x434, 0);
    gen_pin_pairs!(SCU434, 0x434, 1);
    gen_pin_pairs!(SCU434, 0x434, 2);
    gen_pin_pairs!(SCU434, 0x434, 3);
    gen_pin_pairs!(SCU434, 0x434, 4);
    gen_pin_pairs!(SCU434, 0x434, 5);
    gen_pin_pairs!(SCU434, 0x434, 6);
    gen_pin_pairs!(SCU434, 0x434, 7);
    gen_pin_pairs!(SCU434, 0x434, 8);
    gen_pin_pairs!(SCU434, 0x434, 9);
    gen_pin_pairs!(SCU434, 0x434, 10);
    gen_pin_pairs!(SCU434, 0x434, 11);
    gen_pin_pairs!(SCU434, 0x434, 12);
    gen_pin_pairs!(SCU434, 0x434, 13);
    gen_pin_pairs!(SCU434, 0x434, 14);
    gen_pin_pairs!(SCU434, 0x434, 15);
    gen_pin_pairs!(SCU434, 0x434, 16);
    gen_pin_pairs!(SCU434, 0x434, 17);
    gen_pin_pairs!(SCU434, 0x434, 18);
    gen_pin_pairs!(SCU434, 0x434, 19);
    gen_pin_pairs!(SCU434, 0x434, 20);
    gen_pin_pairs!(SCU434, 0x434, 21);
    gen_pin_pairs!(SCU434, 0x434, 22);
    gen_pin_pairs!(SCU434, 0x434, 23);
    gen_pin_pairs!(SCU434, 0x434, 24);
    gen_pin_pairs!(SCU434, 0x434, 25);
    gen_pin_pairs!(SCU434, 0x434, 26);
    gen_pin_pairs!(SCU434, 0x434, 27);
    gen_pin_pairs!(SCU434, 0x434, 28);
    gen_pin_pairs!(SCU434, 0x434, 29);
    gen_pin_pairs!(SCU434, 0x434, 30);
    gen_pin_pairs!(SCU434, 0x434, 31);

    gen_pin_pairs!(SCU438, 0x438, 0);
    gen_pin_pairs!(SCU438, 0x438, 1);
    gen_pin_pairs!(SCU438, 0x438, 2);
    gen_pin_pairs!(SCU438, 0x438, 3);
    gen_pin_pairs!(SCU438, 0x438, 4);
    gen_pin_pairs!(SCU438, 0x438, 5);
    gen_pin_pairs!(SCU438, 0x438, 6);
    gen_pin_pairs!(SCU438, 0x438, 7);
    gen_pin_pairs!(SCU438, 0x438, 8);
    gen_pin_pairs!(SCU438, 0x438, 9);
    gen_pin_pairs!(SCU438, 0x438, 10);
    gen_pin_pairs!(SCU438, 0x438, 11);
    gen_pin_pairs!(SCU438, 0x438, 12);
    gen_pin_pairs!(SCU438, 0x438, 13);
    gen_pin_pairs!(SCU438, 0x438, 14);
    gen_pin_pairs!(SCU438, 0x438, 15);
    gen_pin_pairs!(SCU438, 0x438, 16);
    gen_pin_pairs!(SCU438, 0x438, 17);
    gen_pin_pairs!(SCU438, 0x438, 18);
    gen_pin_pairs!(SCU438, 0x438, 19);
    gen_pin_pairs!(SCU438, 0x438, 20);
    gen_pin_pairs!(SCU438, 0x438, 21);
    gen_pin_pairs!(SCU438, 0x438, 22);
    gen_pin_pairs!(SCU438, 0x438, 23);
    gen_pin_pairs!(SCU438, 0x438, 24);
    gen_pin_pairs!(SCU438, 0x438, 25);
    gen_pin_pairs!(SCU438, 0x438, 26);
    gen_pin_pairs!(SCU438, 0x438, 27);
    gen_pin_pairs!(SCU438, 0x438, 28);
    gen_pin_pairs!(SCU438, 0x438, 29);
    gen_pin_pairs!(SCU438, 0x438, 30);
    gen_pin_pairs!(SCU438, 0x438, 31);

    gen_pin_pairs!(SCU43C, 0x43C, 0);
    gen_pin_pairs!(SCU43C, 0x43C, 1);
    gen_pin_pairs!(SCU43C, 0x43C, 2);
    gen_pin_pairs!(SCU43C, 0x43C, 3);
    gen_pin_pairs!(SCU43C, 0x43C, 4);
    gen_pin_pairs!(SCU43C, 0x43C, 5);
    gen_pin_pairs!(SCU43C, 0x43C, 6);
    gen_pin_pairs!(SCU43C, 0x43C, 7);
    gen_pin_pairs!(SCU43C, 0x43C, 8);
    gen_pin_pairs!(SCU43C, 0x43C, 9);
    gen_pin_pairs!(SCU43C, 0x43C, 10);
    gen_pin_pairs!(SCU43C, 0x43C, 11);
    gen_pin_pairs!(SCU43C, 0x43C, 12);
    gen_pin_pairs!(SCU43C, 0x43C, 13);
    gen_pin_pairs!(SCU43C, 0x43C, 14);
    gen_pin_pairs!(SCU43C, 0x43C, 15);
    gen_pin_pairs!(SCU43C, 0x43C, 16);
    gen_pin_pairs!(SCU43C, 0x43C, 17);
    gen_pin_pairs!(SCU43C, 0x43C, 18);
    gen_pin_pairs!(SCU43C, 0x43C, 19);
    gen_pin_pairs!(SCU43C, 0x43C, 20);
    gen_pin_pairs!(SCU43C, 0x43C, 21);
    gen_pin_pairs!(SCU43C, 0x43C, 22);
    gen_pin_pairs!(SCU43C, 0x43C, 23);
    gen_pin_pairs!(SCU43C, 0x43C, 24);
    gen_pin_pairs!(SCU43C, 0x43C, 25);
    gen_pin_pairs!(SCU43C, 0x43C, 26);
    gen_pin_pairs!(SCU43C, 0x43C, 27);
    gen_pin_pairs!(SCU43C, 0x43C, 28);
    gen_pin_pairs!(SCU43C, 0x43C, 29);
    gen_pin_pairs!(SCU43C, 0x43C, 30);
    gen_pin_pairs!(SCU43C, 0x43C, 31);

    gen_pin_pairs!(SCU440, 0x440, 0);
    gen_pin_pairs!(SCU440, 0x440, 1);
    gen_pin_pairs!(SCU440, 0x440, 2);
    gen_pin_pairs!(SCU440, 0x440, 3);
    gen_pin_pairs!(SCU440, 0x440, 4);
    gen_pin_pairs!(SCU440, 0x440, 5);
    gen_pin_pairs!(SCU440, 0x440, 6);
    gen_pin_pairs!(SCU440, 0x440, 7);
    gen_pin_pairs!(SCU440, 0x440, 8);
    gen_pin_pairs!(SCU440, 0x440, 9);
    gen_pin_pairs!(SCU440, 0x440, 10);
    gen_pin_pairs!(SCU440, 0x440, 11);
    gen_pin_pairs!(SCU440, 0x440, 12);
    gen_pin_pairs!(SCU440, 0x440, 13);
    gen_pin_pairs!(SCU440, 0x440, 14);
    gen_pin_pairs!(SCU440, 0x440, 15);
    gen_pin_pairs!(SCU440, 0x440, 16);
    gen_pin_pairs!(SCU440, 0x440, 17);
    gen_pin_pairs!(SCU440, 0x440, 18);
    gen_pin_pairs!(SCU440, 0x440, 19);
    gen_pin_pairs!(SCU440, 0x440, 20);
    gen_pin_pairs!(SCU440, 0x440, 21);
    gen_pin_pairs!(SCU440, 0x440, 22);
    gen_pin_pairs!(SCU440, 0x440, 23);
    gen_pin_pairs!(SCU440, 0x440, 24);
    gen_pin_pairs!(SCU440, 0x440, 25);
    gen_pin_pairs!(SCU440, 0x440, 26);
    gen_pin_pairs!(SCU440, 0x440, 27);
    gen_pin_pairs!(SCU440, 0x440, 28);
    gen_pin_pairs!(SCU440, 0x440, 29);
    gen_pin_pairs!(SCU440, 0x440, 30);
    gen_pin_pairs!(SCU440, 0x440, 31);

    gen_pin_pairs!(SCU444, 0x444, 0);
    gen_pin_pairs!(SCU444, 0x444, 1);
    gen_pin_pairs!(SCU444, 0x444, 2);
    gen_pin_pairs!(SCU444, 0x444, 3);
    gen_pin_pairs!(SCU444, 0x444, 4);
    gen_pin_pairs!(SCU444, 0x444, 5);
    gen_pin_pairs!(SCU444, 0x444, 6);
    gen_pin_pairs!(SCU444, 0x444, 7);
    gen_pin_pairs!(SCU444, 0x444, 8);
    gen_pin_pairs!(SCU444, 0x444, 9);
    gen_pin_pairs!(SCU444, 0x444, 10);
    gen_pin_pairs!(SCU444, 0x444, 11);
    gen_pin_pairs!(SCU444, 0x444, 12);
    gen_pin_pairs!(SCU444, 0x444, 13);
    gen_pin_pairs!(SCU444, 0x444, 14);
    gen_pin_pairs!(SCU444, 0x444, 15);
    gen_pin_pairs!(SCU444, 0x444, 16);
    gen_pin_pairs!(SCU444, 0x444, 17);
    gen_pin_pairs!(SCU444, 0x444, 18);
    gen_pin_pairs!(SCU444, 0x444, 19);
    gen_pin_pairs!(SCU444, 0x444, 20);
    gen_pin_pairs!(SCU444, 0x444, 21);
    gen_pin_pairs!(SCU444, 0x444, 22);
    gen_pin_pairs!(SCU444, 0x444, 23);
    gen_pin_pairs!(SCU444, 0x444, 24);
    gen_pin_pairs!(SCU444, 0x444, 25);
    gen_pin_pairs!(SCU444, 0x444, 26);
    gen_pin_pairs!(SCU444, 0x444, 27);
    gen_pin_pairs!(SCU444, 0x444, 28);
    gen_pin_pairs!(SCU444, 0x444, 29);
    gen_pin_pairs!(SCU444, 0x444, 30);
    gen_pin_pairs!(SCU444, 0x444, 31);

    gen_pin_pairs!(SCU448, 0x448, 0);
    gen_pin_pairs!(SCU448, 0x448, 1);
    gen_pin_pairs!(SCU448, 0x448, 2);
    gen_pin_pairs!(SCU448, 0x448, 3);
    gen_pin_pairs!(SCU448, 0x448, 4);
    gen_pin_pairs!(SCU448, 0x448, 5);
    gen_pin_pairs!(SCU448, 0x448, 6);
    gen_pin_pairs!(SCU448, 0x448, 7);
    gen_pin_pairs!(SCU448, 0x448, 8);
    gen_pin_pairs!(SCU448, 0x448, 9);
    gen_pin_pairs!(SCU448, 0x448, 10);
    gen_pin_pairs!(SCU448, 0x448, 11);
    gen_pin_pairs!(SCU448, 0x448, 12);
    gen_pin_pairs!(SCU448, 0x448, 13);
    gen_pin_pairs!(SCU448, 0x448, 14);
    gen_pin_pairs!(SCU448, 0x448, 15);
    gen_pin_pairs!(SCU448, 0x448, 16);
    gen_pin_pairs!(SCU448, 0x448, 17);
    gen_pin_pairs!(SCU448, 0x448, 18);
    gen_pin_pairs!(SCU448, 0x448, 19);
    gen_pin_pairs!(SCU448, 0x448, 20);
    gen_pin_pairs!(SCU448, 0x448, 21);
    gen_pin_pairs!(SCU448, 0x448, 22);
    gen_pin_pairs!(SCU448, 0x448, 23);
    gen_pin_pairs!(SCU448, 0x448, 24);
    gen_pin_pairs!(SCU448, 0x448, 25);
    gen_pin_pairs!(SCU448, 0x448, 26);
    gen_pin_pairs!(SCU448, 0x448, 27);
    gen_pin_pairs!(SCU448, 0x448, 28);
    gen_pin_pairs!(SCU448, 0x448, 29);
    gen_pin_pairs!(SCU448, 0x448, 30);
    gen_pin_pairs!(SCU448, 0x448, 31);

    gen_pin_pairs!(SCU44C, 0x44C, 0);
    gen_pin_pairs!(SCU44C, 0x44C, 1);
    gen_pin_pairs!(SCU44C, 0x44C, 2);
    gen_pin_pairs!(SCU44C, 0x44C, 3);
    gen_pin_pairs!(SCU44C, 0x44C, 4);
    gen_pin_pairs!(SCU44C, 0x44C, 5);
    gen_pin_pairs!(SCU44C, 0x44C, 6);
    gen_pin_pairs!(SCU44C, 0x44C, 7);
    gen_pin_pairs!(SCU44C, 0x44C, 8);
    gen_pin_pairs!(SCU44C, 0x44C, 9);
    gen_pin_pairs!(SCU44C, 0x44C, 10);
    gen_pin_pairs!(SCU44C, 0x44C, 11);
    gen_pin_pairs!(SCU44C, 0x44C, 12);
    gen_pin_pairs!(SCU44C, 0x44C, 13);
    gen_pin_pairs!(SCU44C, 0x44C, 14);
    gen_pin_pairs!(SCU44C, 0x44C, 15);
    gen_pin_pairs!(SCU44C, 0x44C, 16);
    gen_pin_pairs!(SCU44C, 0x44C, 17);
    gen_pin_pairs!(SCU44C, 0x44C, 18);
    gen_pin_pairs!(SCU44C, 0x44C, 19);
    gen_pin_pairs!(SCU44C, 0x44C, 20);
    gen_pin_pairs!(SCU44C, 0x44C, 21);
    gen_pin_pairs!(SCU44C, 0x44C, 22);
    gen_pin_pairs!(SCU44C, 0x44C, 23);
    gen_pin_pairs!(SCU44C, 0x44C, 24);
    gen_pin_pairs!(SCU44C, 0x44C, 25);
    gen_pin_pairs!(SCU44C, 0x44C, 26);
    gen_pin_pairs!(SCU44C, 0x44C, 27);
    gen_pin_pairs!(SCU44C, 0x44C, 28);
    gen_pin_pairs!(SCU44C, 0x44C, 29);
    gen_pin_pairs!(SCU44C, 0x44C, 30);
    gen_pin_pairs!(SCU44C, 0x44C, 31);

    gen_pin_pairs!(SCU450, 0x450, 0);
    gen_pin_pairs!(SCU450, 0x450, 1);
    gen_pin_pairs!(SCU450, 0x450, 2);
    gen_pin_pairs!(SCU450, 0x450, 3);
    gen_pin_pairs!(SCU450, 0x450, 4);
    gen_pin_pairs!(SCU450, 0x450, 5);
    gen_pin_pairs!(SCU450, 0x450, 6);
    gen_pin_pairs!(SCU450, 0x450, 7);
    gen_pin_pairs!(SCU450, 0x450, 8);
    gen_pin_pairs!(SCU450, 0x450, 9);
    gen_pin_pairs!(SCU450, 0x450, 10);
    gen_pin_pairs!(SCU450, 0x450, 11);
    gen_pin_pairs!(SCU450, 0x450, 12);
    gen_pin_pairs!(SCU450, 0x450, 13);
    gen_pin_pairs!(SCU450, 0x450, 14);
    gen_pin_pairs!(SCU450, 0x450, 15);
    gen_pin_pairs!(SCU450, 0x450, 16);
    gen_pin_pairs!(SCU450, 0x450, 17);
    gen_pin_pairs!(SCU450, 0x450, 18);
    gen_pin_pairs!(SCU450, 0x450, 19);
    gen_pin_pairs!(SCU450, 0x450, 20);
    gen_pin_pairs!(SCU450, 0x450, 21);
    gen_pin_pairs!(SCU450, 0x450, 22);
    gen_pin_pairs!(SCU450, 0x450, 23);
    gen_pin_pairs!(SCU450, 0x450, 24);
    gen_pin_pairs!(SCU450, 0x450, 25);
    gen_pin_pairs!(SCU450, 0x450, 26);
    gen_pin_pairs!(SCU450, 0x450, 27);
    gen_pin_pairs!(SCU450, 0x450, 28);
    gen_pin_pairs!(SCU450, 0x450, 29);
    gen_pin_pairs!(SCU450, 0x450, 30);
    gen_pin_pairs!(SCU450, 0x450, 31);

    gen_pin_pairs!(SCU454, 0x454, 0);
    gen_pin_pairs!(SCU454, 0x454, 1);
    gen_pin_pairs!(SCU454, 0x454, 2);
    gen_pin_pairs!(SCU454, 0x454, 3);
    gen_pin_pairs!(SCU454, 0x454, 4);
    gen_pin_pairs!(SCU454, 0x454, 5);
    gen_pin_pairs!(SCU454, 0x454, 6);
    gen_pin_pairs!(SCU454, 0x454, 7);
    gen_pin_pairs!(SCU454, 0x454, 8);
    gen_pin_pairs!(SCU454, 0x454, 9);
    gen_pin_pairs!(SCU454, 0x454, 10);
    gen_pin_pairs!(SCU454, 0x454, 11);
    gen_pin_pairs!(SCU454, 0x454, 12);
    gen_pin_pairs!(SCU454, 0x454, 13);
    gen_pin_pairs!(SCU454, 0x454, 14);
    gen_pin_pairs!(SCU454, 0x454, 15);
    gen_pin_pairs!(SCU454, 0x454, 16);
    gen_pin_pairs!(SCU454, 0x454, 17);
    gen_pin_pairs!(SCU454, 0x454, 18);
    gen_pin_pairs!(SCU454, 0x454, 19);
    gen_pin_pairs!(SCU454, 0x454, 20);
    gen_pin_pairs!(SCU454, 0x454, 21);
    gen_pin_pairs!(SCU454, 0x454, 22);
    gen_pin_pairs!(SCU454, 0x454, 23);
    gen_pin_pairs!(SCU454, 0x454, 24);
    gen_pin_pairs!(SCU454, 0x454, 25);
    gen_pin_pairs!(SCU454, 0x454, 26);
    gen_pin_pairs!(SCU454, 0x454, 27);
    gen_pin_pairs!(SCU454, 0x454, 28);
    gen_pin_pairs!(SCU454, 0x454, 29);
    gen_pin_pairs!(SCU454, 0x454, 30);
    gen_pin_pairs!(SCU454, 0x454, 31);

    gen_pin_pairs!(SCU458, 0x458, 0);
    gen_pin_pairs!(SCU458, 0x458, 1);
    gen_pin_pairs!(SCU458, 0x458, 2);
    gen_pin_pairs!(SCU458, 0x458, 3);
    gen_pin_pairs!(SCU458, 0x458, 4);
    gen_pin_pairs!(SCU458, 0x458, 5);
    gen_pin_pairs!(SCU458, 0x458, 6);
    gen_pin_pairs!(SCU458, 0x458, 7);
    gen_pin_pairs!(SCU458, 0x458, 8);
    gen_pin_pairs!(SCU458, 0x458, 9);
    gen_pin_pairs!(SCU458, 0x458, 10);
    gen_pin_pairs!(SCU458, 0x458, 11);
    gen_pin_pairs!(SCU458, 0x458, 12);
    gen_pin_pairs!(SCU458, 0x458, 13);
    gen_pin_pairs!(SCU458, 0x458, 14);
    gen_pin_pairs!(SCU458, 0x458, 15);
    gen_pin_pairs!(SCU458, 0x458, 16);
    gen_pin_pairs!(SCU458, 0x458, 17);
    gen_pin_pairs!(SCU458, 0x458, 18);
    gen_pin_pairs!(SCU458, 0x458, 19);
    gen_pin_pairs!(SCU458, 0x458, 20);
    gen_pin_pairs!(SCU458, 0x458, 21);
    gen_pin_pairs!(SCU458, 0x458, 22);
    gen_pin_pairs!(SCU458, 0x458, 23);
    gen_pin_pairs!(SCU458, 0x458, 24);
    gen_pin_pairs!(SCU458, 0x458, 25);
    gen_pin_pairs!(SCU458, 0x458, 26);
    gen_pin_pairs!(SCU458, 0x458, 27);
    gen_pin_pairs!(SCU458, 0x458, 28);
    gen_pin_pairs!(SCU458, 0x458, 29);
    gen_pin_pairs!(SCU458, 0x458, 30);
    gen_pin_pairs!(SCU458, 0x458, 31);

    gen_pin_pairs!(SCU45C, 0x45C, 0);
    gen_pin_pairs!(SCU45C, 0x45C, 1);
    gen_pin_pairs!(SCU45C, 0x45C, 2);
    gen_pin_pairs!(SCU45C, 0x45C, 3);
    gen_pin_pairs!(SCU45C, 0x45C, 4);
    gen_pin_pairs!(SCU45C, 0x45C, 5);
    gen_pin_pairs!(SCU45C, 0x45C, 6);
    gen_pin_pairs!(SCU45C, 0x45C, 7);
    gen_pin_pairs!(SCU45C, 0x45C, 8);
    gen_pin_pairs!(SCU45C, 0x45C, 9);
    gen_pin_pairs!(SCU45C, 0x45C, 10);
    gen_pin_pairs!(SCU45C, 0x45C, 11);
    gen_pin_pairs!(SCU45C, 0x45C, 12);
    gen_pin_pairs!(SCU45C, 0x45C, 13);
    gen_pin_pairs!(SCU45C, 0x45C, 14);
    gen_pin_pairs!(SCU45C, 0x45C, 15);
    gen_pin_pairs!(SCU45C, 0x45C, 16);
    gen_pin_pairs!(SCU45C, 0x45C, 17);
    gen_pin_pairs!(SCU45C, 0x45C, 18);
    gen_pin_pairs!(SCU45C, 0x45C, 19);
    gen_pin_pairs!(SCU45C, 0x45C, 20);
    gen_pin_pairs!(SCU45C, 0x45C, 21);
    gen_pin_pairs!(SCU45C, 0x45C, 22);
    gen_pin_pairs!(SCU45C, 0x45C, 23);
    gen_pin_pairs!(SCU45C, 0x45C, 24);
    gen_pin_pairs!(SCU45C, 0x45C, 25);
    gen_pin_pairs!(SCU45C, 0x45C, 26);
    gen_pin_pairs!(SCU45C, 0x45C, 27);
    gen_pin_pairs!(SCU45C, 0x45C, 28);
    gen_pin_pairs!(SCU45C, 0x45C, 29);
    gen_pin_pairs!(SCU45C, 0x45C, 30);
    gen_pin_pairs!(SCU45C, 0x45C, 31);

    gen_pin_pairs!(SCU460, 0x460, 0);
    gen_pin_pairs!(SCU460, 0x460, 1);
    gen_pin_pairs!(SCU460, 0x460, 2);
    gen_pin_pairs!(SCU460, 0x460, 3);
    gen_pin_pairs!(SCU460, 0x460, 4);
    gen_pin_pairs!(SCU460, 0x460, 5);
    gen_pin_pairs!(SCU460, 0x460, 6);
    gen_pin_pairs!(SCU460, 0x460, 7);
    gen_pin_pairs!(SCU460, 0x460, 8);
    gen_pin_pairs!(SCU460, 0x460, 9);
    gen_pin_pairs!(SCU460, 0x460, 10);
    gen_pin_pairs!(SCU460, 0x460, 11);
    gen_pin_pairs!(SCU460, 0x460, 12);
    gen_pin_pairs!(SCU460, 0x460, 13);
    gen_pin_pairs!(SCU460, 0x460, 14);
    gen_pin_pairs!(SCU460, 0x460, 15);
    gen_pin_pairs!(SCU460, 0x460, 16);
    gen_pin_pairs!(SCU460, 0x460, 17);
    gen_pin_pairs!(SCU460, 0x460, 18);
    gen_pin_pairs!(SCU460, 0x460, 19);
    gen_pin_pairs!(SCU460, 0x460, 20);
    gen_pin_pairs!(SCU460, 0x460, 21);
    gen_pin_pairs!(SCU460, 0x460, 22);
    gen_pin_pairs!(SCU460, 0x460, 23);
    gen_pin_pairs!(SCU460, 0x460, 24);
    gen_pin_pairs!(SCU460, 0x460, 25);
    gen_pin_pairs!(SCU460, 0x460, 26);
    gen_pin_pairs!(SCU460, 0x460, 27);
    gen_pin_pairs!(SCU460, 0x460, 28);
    gen_pin_pairs!(SCU460, 0x460, 29);
    gen_pin_pairs!(SCU460, 0x460, 30);
    gen_pin_pairs!(SCU460, 0x460, 31);
}

/// I2C1 pin group: SCL1/SDA1 mux selection on SCU408[10:8]/SCU408[14:12].
pub const PINCTRL_I2C1: &[PinctrlPin] = &[CLR_PIN_SCU408_10, CLR_PIN_SCU408_9, PIN_SCU408_8, CLR_PIN_SCU408_14, CLR_PIN_SCU408_13, PIN_SCU408_12];

/// I2C2 pin group: SCL2/SDA2 mux selection on SCU408[18:16]/SCU408[22:20].
pub const PINCTRL_I2C2: &[PinctrlPin] = &[CLR_PIN_SCU408_18, CLR_PIN_SCU408_17, PIN_SCU408_16, CLR_PIN_SCU408_22, CLR_PIN_SCU408_21, PIN_SCU408_20];

/// GPIOA pin groups: GPIO mux selection.
pub const PINCTRL_GPIOA0: &[PinctrlPin] = &[CLR_PIN_SCU400_2, CLR_PIN_SCU400_1, CLR_PIN_SCU400_0];
pub const PINCTRL_GPIOA1: &[PinctrlPin] = &[CLR_PIN_SCU400_6, CLR_PIN_SCU400_5, CLR_PIN_SCU400_4];
pub const PINCTRL_GPIOA2: &[PinctrlPin] = &[CLR_PIN_SCU400_10, CLR_PIN_SCU400_9, CLR_PIN_SCU400_8];
pub const PINCTRL_GPIOA3: &[PinctrlPin] = &[CLR_PIN_SCU400_14, CLR_PIN_SCU400_13, CLR_PIN_SCU400_12];
pub const PINCTRL_GPIOA4: &[PinctrlPin] = &[CLR_PIN_SCU400_18, CLR_PIN_SCU400_17, CLR_PIN_SCU400_16];
pub const PINCTRL_GPIOA5: &[PinctrlPin] = &[CLR_PIN_SCU400_22, CLR_PIN_SCU400_21, CLR_PIN_SCU400_20];
pub const PINCTRL_GPIOA6: &[PinctrlPin] = &[CLR_PIN_SCU400_26, CLR_PIN_SCU400_25, CLR_PIN_SCU400_24];
pub const PINCTRL_GPIOA7: &[PinctrlPin] = &[CLR_PIN_SCU400_30, CLR_PIN_SCU400_29, CLR_PIN_SCU400_28];

/// GPIOB pin groups: GPIO mux selection.
pub const PINCTRL_GPIOB0: &[PinctrlPin] = &[CLR_PIN_SCU404_2, CLR_PIN_SCU404_1, CLR_PIN_SCU404_0];
pub const PINCTRL_GPIOB1: &[PinctrlPin] = &[CLR_PIN_SCU404_6, CLR_PIN_SCU404_5, CLR_PIN_SCU404_4];
pub const PINCTRL_GPIOB2: &[PinctrlPin] =
    &[CLR_PIN_SCU404_10, CLR_PIN_SCU404_9, CLR_PIN_SCU404_8];
pub const PINCTRL_GPIOB3: &[PinctrlPin] =
    &[CLR_PIN_SCU404_14, CLR_PIN_SCU404_13, CLR_PIN_SCU404_12];
pub const PINCTRL_GPIOB4: &[PinctrlPin] =
    &[CLR_PIN_SCU404_18, CLR_PIN_SCU404_17, CLR_PIN_SCU404_16];
pub const PINCTRL_GPIOB5: &[PinctrlPin] =
    &[CLR_PIN_SCU404_22, CLR_PIN_SCU404_21, CLR_PIN_SCU404_20];
pub const PINCTRL_GPIOB6: &[PinctrlPin] =
    &[CLR_PIN_SCU404_26, CLR_PIN_SCU404_25, CLR_PIN_SCU404_24];
pub const PINCTRL_GPIOB7: &[PinctrlPin] =
    &[CLR_PIN_SCU404_30, CLR_PIN_SCU404_29, CLR_PIN_SCU404_28];

/// GPIOC pin groups: GPIO mux selection.
pub const PINCTRL_GPIOC0: &[PinctrlPin] =
    &[CLR_PIN_SCU408_2, CLR_PIN_SCU408_1, CLR_PIN_SCU408_0];
pub const PINCTRL_GPIOC1: &[PinctrlPin] =
    &[CLR_PIN_SCU408_6, CLR_PIN_SCU408_5, CLR_PIN_SCU408_4];
pub const PINCTRL_GPIOC2: &[PinctrlPin] =
    &[CLR_PIN_SCU408_10, CLR_PIN_SCU408_9, CLR_PIN_SCU408_8];
pub const PINCTRL_GPIOC3: &[PinctrlPin] =
    &[CLR_PIN_SCU408_14, CLR_PIN_SCU408_13, CLR_PIN_SCU408_12];
pub const PINCTRL_GPIOC4: &[PinctrlPin] =
    &[CLR_PIN_SCU408_18, CLR_PIN_SCU408_17, CLR_PIN_SCU408_16];
pub const PINCTRL_GPIOC5: &[PinctrlPin] =
    &[CLR_PIN_SCU408_22, CLR_PIN_SCU408_21, CLR_PIN_SCU408_20];
pub const PINCTRL_GPIOC6: &[PinctrlPin] =
    &[CLR_PIN_SCU408_26, CLR_PIN_SCU408_25, CLR_PIN_SCU408_24];
pub const PINCTRL_GPIOC7: &[PinctrlPin] =
    &[CLR_PIN_SCU408_30, CLR_PIN_SCU408_29, CLR_PIN_SCU408_28];

/// GPIOD pin groups: GPIO mux selection.


/// GPIOE pin groups: GPIO mux selection.
pub const PINCTRL_GPIOE4: &[PinctrlPin] = &[CLR_PIN_SCU410_18, CLR_PIN_SCU410_17, CLR_PIN_SCU410_16];
pub const PINCTRL_GPIOE5: &[PinctrlPin] = &[CLR_PIN_SCU410_22, CLR_PIN_SCU410_21, CLR_PIN_SCU410_20];
pub const PINCTRL_GPIOE6: &[PinctrlPin] = &[CLR_PIN_SCU410_26, CLR_PIN_SCU410_25, CLR_PIN_SCU410_24];
pub const PINCTRL_GPIOE7: &[PinctrlPin] = &[CLR_PIN_SCU410_30, CLR_PIN_SCU410_29, CLR_PIN_SCU410_28];

/// GPIOF pin groups: GPIO mux selection.
pub const PINCTRL_GPIOF0: &[PinctrlPin] = &[CLR_PIN_SCU414_2, CLR_PIN_SCU414_1, CLR_PIN_SCU414_0];
pub const PINCTRL_GPIOF1: &[PinctrlPin] = &[CLR_PIN_SCU414_6, CLR_PIN_SCU414_5, CLR_PIN_SCU414_4];
pub const PINCTRL_GPIOF2: &[PinctrlPin] =
    &[CLR_PIN_SCU414_10, CLR_PIN_SCU414_9, CLR_PIN_SCU414_8];
pub const PINCTRL_GPIOF3: &[PinctrlPin] =
    &[CLR_PIN_SCU414_14, CLR_PIN_SCU414_13, CLR_PIN_SCU414_12];
pub const PINCTRL_GPIOF4: &[PinctrlPin] =
    &[CLR_PIN_SCU414_18, CLR_PIN_SCU414_17, CLR_PIN_SCU414_16];
pub const PINCTRL_GPIOF5: &[PinctrlPin] =
    &[CLR_PIN_SCU414_22, CLR_PIN_SCU414_21, CLR_PIN_SCU414_20];
pub const PINCTRL_GPIOF6: &[PinctrlPin] =
    &[CLR_PIN_SCU414_26, CLR_PIN_SCU414_25, CLR_PIN_SCU414_24];
pub const PINCTRL_GPIOF7: &[PinctrlPin] =
    &[CLR_PIN_SCU414_30, CLR_PIN_SCU414_29, CLR_PIN_SCU414_28];

/// GPIOG pin groups: GPIO mux selection.
pub const PINCTRL_GPIOG0: &[PinctrlPin] =
    &[CLR_PIN_SCU418_2, CLR_PIN_SCU418_1, CLR_PIN_SCU418_0];
pub const PINCTRL_GPIOG1: &[PinctrlPin] =
    &[CLR_PIN_SCU418_6, CLR_PIN_SCU418_5, CLR_PIN_SCU418_4];
pub const PINCTRL_GPIOG2: &[PinctrlPin] =
    &[CLR_PIN_SCU418_10, CLR_PIN_SCU418_9, CLR_PIN_SCU418_8];
pub const PINCTRL_GPIOG3: &[PinctrlPin] =
    &[CLR_PIN_SCU418_14, CLR_PIN_SCU418_13, CLR_PIN_SCU418_12];
pub const PINCTRL_GPIOG4: &[PinctrlPin] =
    &[CLR_PIN_SCU418_18, CLR_PIN_SCU418_17, CLR_PIN_SCU418_16];
pub const PINCTRL_GPIOG5: &[PinctrlPin] =
    &[CLR_PIN_SCU418_22, CLR_PIN_SCU418_21, CLR_PIN_SCU418_20];
pub const PINCTRL_GPIOG6: &[PinctrlPin] =
    &[CLR_PIN_SCU418_26, CLR_PIN_SCU418_25, CLR_PIN_SCU418_24];
pub const PINCTRL_GPIOG7: &[PinctrlPin] =
    &[CLR_PIN_SCU418_30, CLR_PIN_SCU418_29, CLR_PIN_SCU418_28];

/// GPIOH pin groups: GPIO mux selection.
pub const PINCTRL_GPIOH0: &[PinctrlPin] =
    &[CLR_PIN_SCU41C_2, CLR_PIN_SCU41C_1, CLR_PIN_SCU41C_0];
pub const PINCTRL_GPIOH1: &[PinctrlPin] =
    &[CLR_PIN_SCU41C_6, CLR_PIN_SCU41C_5, CLR_PIN_SCU41C_4];
pub const PINCTRL_GPIOH2: &[PinctrlPin] =
    &[CLR_PIN_SCU41C_10, CLR_PIN_SCU41C_9, CLR_PIN_SCU41C_8];
pub const PINCTRL_GPIOH3: &[PinctrlPin] =
    &[CLR_PIN_SCU41C_14, CLR_PIN_SCU41C_13, CLR_PIN_SCU41C_12];

/// GPIOI pin groups: GPIO mux selection.


/// GPIOJ pin groups: GPIO mux selection.


/// GPIOK pin groups: GPIO mux selection.


/// GPIOL pin groups: GPIO mux selection.


/// GPIOM pin groups: GPIO mux selection.


/// GPION pin groups: GPIO mux selection.


/// GPIOO pin groups: GPIO mux selection.
pub const PINCTRL_GPIOO6: &[PinctrlPin] =
    &[CLR_PIN_SCU438_26, CLR_PIN_SCU438_25, CLR_PIN_SCU438_24];
pub const PINCTRL_GPIOO7: &[PinctrlPin] =
    &[CLR_PIN_SCU438_30, CLR_PIN_SCU438_29, CLR_PIN_SCU438_28];

/// GPIOP pin groups: GPIO mux selection.
pub const PINCTRL_GPIOP0: &[PinctrlPin] =
    &[CLR_PIN_SCU43C_2, CLR_PIN_SCU43C_1, CLR_PIN_SCU43C_0];
pub const PINCTRL_GPIOP1: &[PinctrlPin] = &[CLR_PIN_SCU43C_6, CLR_PIN_SCU43C_5, CLR_PIN_SCU43C_4];
pub const PINCTRL_GPIOP6: &[PinctrlPin] =
    &[CLR_PIN_SCU43C_26, CLR_PIN_SCU43C_25, CLR_PIN_SCU43C_24];
pub const PINCTRL_GPIOP7: &[PinctrlPin] =
    &[CLR_PIN_SCU43C_30, CLR_PIN_SCU43C_29, CLR_PIN_SCU43C_28];

/// GPIOQ pin groups: GPIO mux selection.
pub const PINCTRL_GPIOQ0: &[PinctrlPin] = &[CLR_PIN_SCU440_2, CLR_PIN_SCU440_1, CLR_PIN_SCU440_0];
pub const PINCTRL_GPIOQ1: &[PinctrlPin] = &[CLR_PIN_SCU440_6, CLR_PIN_SCU440_5, CLR_PIN_SCU440_4];
pub const PINCTRL_GPIOQ2: &[PinctrlPin] = &[CLR_PIN_SCU440_10, CLR_PIN_SCU440_9, CLR_PIN_SCU440_8];
pub const PINCTRL_GPIOQ3: &[PinctrlPin] = &[CLR_PIN_SCU440_14, CLR_PIN_SCU440_13, CLR_PIN_SCU440_12];
pub const PINCTRL_GPIOQ4: &[PinctrlPin] = &[CLR_PIN_SCU440_18, CLR_PIN_SCU440_17, CLR_PIN_SCU440_16];
pub const PINCTRL_GPIOQ5: &[PinctrlPin] = &[CLR_PIN_SCU440_22, CLR_PIN_SCU440_21, CLR_PIN_SCU440_20];
pub const PINCTRL_GPIOQ6: &[PinctrlPin] = &[CLR_PIN_SCU440_26, CLR_PIN_SCU440_25, CLR_PIN_SCU440_24];
pub const PINCTRL_GPIOQ7: &[PinctrlPin] = &[CLR_PIN_SCU440_30, CLR_PIN_SCU440_29, CLR_PIN_SCU440_28];

/// GPIOR pin groups: GPIO mux selection.
pub const PINCTRL_GPIOR0: &[PinctrlPin] = &[CLR_PIN_SCU444_2, CLR_PIN_SCU444_1, CLR_PIN_SCU444_0];
pub const PINCTRL_GPIOR1: &[PinctrlPin] = &[CLR_PIN_SCU444_6, CLR_PIN_SCU444_5, CLR_PIN_SCU444_4];
pub const PINCTRL_GPIOR2: &[PinctrlPin] = &[CLR_PIN_SCU444_10, CLR_PIN_SCU444_9, CLR_PIN_SCU444_8];

/// GPIOS pin groups: GPIO mux selection.
pub const PINCTRL_GPIOS0: &[PinctrlPin] = &[CLR_PIN_SCU448_2, CLR_PIN_SCU448_1, CLR_PIN_SCU448_0];
pub const PINCTRL_GPIOS1: &[PinctrlPin] = &[CLR_PIN_SCU448_6, CLR_PIN_SCU448_5, CLR_PIN_SCU448_4];
pub const PINCTRL_GPIOS2: &[PinctrlPin] = &[CLR_PIN_SCU448_10, CLR_PIN_SCU448_9, CLR_PIN_SCU448_8];
pub const PINCTRL_GPIOS3: &[PinctrlPin] = &[CLR_PIN_SCU448_14, CLR_PIN_SCU448_13, CLR_PIN_SCU448_12];
pub const PINCTRL_GPIOS4: &[PinctrlPin] = &[CLR_PIN_SCU448_18, CLR_PIN_SCU448_17, CLR_PIN_SCU448_16];
pub const PINCTRL_GPIOS5: &[PinctrlPin] = &[CLR_PIN_SCU448_22, CLR_PIN_SCU448_21, CLR_PIN_SCU448_20];
pub const PINCTRL_GPIOS6: &[PinctrlPin] = &[CLR_PIN_SCU448_26, CLR_PIN_SCU448_25, CLR_PIN_SCU448_24];
pub const PINCTRL_GPIOS7: &[PinctrlPin] = &[CLR_PIN_SCU448_30, CLR_PIN_SCU448_29, CLR_PIN_SCU448_28];

/// GPIOT pin groups: GPIO mux selection.
pub const PINCTRL_GPIOT0: &[PinctrlPin] = &[CLR_PIN_SCU44C_2, CLR_PIN_SCU44C_1, CLR_PIN_SCU44C_0];
pub const PINCTRL_GPIOT1: &[PinctrlPin] = &[CLR_PIN_SCU44C_6, CLR_PIN_SCU44C_5, CLR_PIN_SCU44C_4];
pub const PINCTRL_GPIOT2: &[PinctrlPin] = &[CLR_PIN_SCU44C_10, CLR_PIN_SCU44C_9, CLR_PIN_SCU44C_8];
pub const PINCTRL_GPIOT3: &[PinctrlPin] = &[CLR_PIN_SCU44C_14, CLR_PIN_SCU44C_13, CLR_PIN_SCU44C_12];
pub const PINCTRL_GPIOT4: &[PinctrlPin] = &[CLR_PIN_SCU44C_18, CLR_PIN_SCU44C_17, CLR_PIN_SCU44C_16];
pub const PINCTRL_GPIOT5: &[PinctrlPin] = &[CLR_PIN_SCU44C_22, CLR_PIN_SCU44C_21, CLR_PIN_SCU44C_20];
pub const PINCTRL_GPIOT6: &[PinctrlPin] = &[CLR_PIN_SCU44C_26, CLR_PIN_SCU44C_25, CLR_PIN_SCU44C_24];
pub const PINCTRL_GPIOT7: &[PinctrlPin] = &[CLR_PIN_SCU44C_30, CLR_PIN_SCU44C_29, CLR_PIN_SCU44C_28];

/// GPIOU pin groups: GPIO mux selection.
pub const PINCTRL_GPIOU0: &[PinctrlPin] = &[CLR_PIN_SCU450_2, CLR_PIN_SCU450_1, CLR_PIN_SCU450_0];
pub const PINCTRL_GPIOU1: &[PinctrlPin] = &[CLR_PIN_SCU450_6, CLR_PIN_SCU450_5, CLR_PIN_SCU450_4];
pub const PINCTRL_GPIOU2: &[PinctrlPin] = &[CLR_PIN_SCU450_10, CLR_PIN_SCU450_9, CLR_PIN_SCU450_8];
pub const PINCTRL_GPIOU3: &[PinctrlPin] = &[CLR_PIN_SCU450_14, CLR_PIN_SCU450_13, CLR_PIN_SCU450_12];
pub const PINCTRL_GPIOU4: &[PinctrlPin] = &[CLR_PIN_SCU450_18, CLR_PIN_SCU450_17, CLR_PIN_SCU450_16];
pub const PINCTRL_GPIOU5: &[PinctrlPin] = &[CLR_PIN_SCU450_22, CLR_PIN_SCU450_21, CLR_PIN_SCU450_20];
pub const PINCTRL_GPIOU6: &[PinctrlPin] = &[CLR_PIN_SCU450_26, CLR_PIN_SCU450_25, CLR_PIN_SCU450_24];
pub const PINCTRL_GPIOU7: &[PinctrlPin] = &[CLR_PIN_SCU450_30, CLR_PIN_SCU450_29, CLR_PIN_SCU450_28];

/// GPIOV pin groups: GPIO mux selection.
pub const PINCTRL_GPIOV0: &[PinctrlPin] = &[CLR_PIN_SCU454_2, CLR_PIN_SCU454_1, CLR_PIN_SCU454_0];
pub const PINCTRL_GPIOV1: &[PinctrlPin] = &[CLR_PIN_SCU454_6, CLR_PIN_SCU454_5, CLR_PIN_SCU454_4];
pub const PINCTRL_GPIOV2: &[PinctrlPin] = &[CLR_PIN_SCU454_10, CLR_PIN_SCU454_9, CLR_PIN_SCU454_8];
pub const PINCTRL_GPIOV3: &[PinctrlPin] = &[CLR_PIN_SCU454_14, CLR_PIN_SCU454_13, CLR_PIN_SCU454_12];
pub const PINCTRL_GPIOV4: &[PinctrlPin] = &[CLR_PIN_SCU454_18, CLR_PIN_SCU454_17, CLR_PIN_SCU454_16];
pub const PINCTRL_GPIOV5: &[PinctrlPin] = &[CLR_PIN_SCU454_22, CLR_PIN_SCU454_21, CLR_PIN_SCU454_20];
pub const PINCTRL_GPIOV6: &[PinctrlPin] = &[CLR_PIN_SCU454_26, CLR_PIN_SCU454_25, CLR_PIN_SCU454_24];

/// GPIOW pin groups: GPIO mux selection.
pub const PINCTRL_GPIOW1: &[PinctrlPin] = &[CLR_PIN_SCU458_6, CLR_PIN_SCU458_5, CLR_PIN_SCU458_4];
pub const PINCTRL_GPIOW2: &[PinctrlPin] = &[CLR_PIN_SCU458_10, CLR_PIN_SCU458_9, CLR_PIN_SCU458_8];

/// GPIOX pin groups: GPIO mux selection.
pub const PINCTRL_GPIOX3: &[PinctrlPin] = &[CLR_PIN_SCU45C_14, CLR_PIN_SCU45C_13, CLR_PIN_SCU45C_12];
pub const PINCTRL_GPIOX4: &[PinctrlPin] = &[CLR_PIN_SCU45C_18, CLR_PIN_SCU45C_17, CLR_PIN_SCU45C_16];
pub const PINCTRL_GPIOX5: &[PinctrlPin] = &[CLR_PIN_SCU45C_22, CLR_PIN_SCU45C_21, CLR_PIN_SCU45C_20];
pub const PINCTRL_GPIOX6: &[PinctrlPin] = &[CLR_PIN_SCU45C_26, CLR_PIN_SCU45C_25, CLR_PIN_SCU45C_24];
pub const PINCTRL_GPIOX7: &[PinctrlPin] = &[CLR_PIN_SCU45C_30, CLR_PIN_SCU45C_29, CLR_PIN_SCU45C_28];

/// GPIOY pin groups: GPIO mux selection.
pub const PINCTRL_GPIOY0: &[PinctrlPin] = &[CLR_PIN_SCU460_2, CLR_PIN_SCU460_1, CLR_PIN_SCU460_0];
pub const PINCTRL_GPIOY1: &[PinctrlPin] = &[CLR_PIN_SCU460_6, CLR_PIN_SCU460_5, CLR_PIN_SCU460_4];
/*
/// FMC quad-SPI pin group: mux selection on SCU430[10:11].
pub const PINCTRL_FMC_QUAD: &[PinctrlPin] = &[PIN_SCU430_10, PIN_SCU430_11];

/// SPI1 quad.
pub const PINCTRL_SPI1_QUAD: &[PinctrlPin] = &[PIN_SCU430_17, PIN_SCU430_18];

/// SPI2 quad.
pub const PINCTRL_SPI2_QUAD: &[PinctrlPin] = &[
    PIN_SCU41C_30,
    PIN_SCU41C_31,
    PIN_SCU430_0,
    PIN_SCU430_1,
    PIN_SCU430_2,
    PIN_SCU430_3,
    PIN_SCU430_4,
];

/// SPIM1
pub const PINCTRL_SPIM1_DEFAULT: &[PinctrlPin] = &[
    // CSIN
    CLR_PIN_SCU410_0,
    CLR_PIN_SCU4B0_0,
    PIN_SCU690_0,
    // CLKIN
    CLR_PIN_SCU410_1,
    CLR_PIN_SCU4B0_1,
    PIN_SCU690_1,
    // MOSIIN
    CLR_PIN_SCU410_2,
    CLR_PIN_SCU4B0_2,
    PIN_SCU690_2,
    // MISOIN
    CLR_PIN_SCU410_3,
    CLR_PIN_SCU4B0_3,
    PIN_SCU690_3,
    // IO2IN
    CLR_PIN_SCU410_4,
    CLR_PIN_SCU4B0_4,
    PIN_SCU690_4,
    // IO3IN
    CLR_PIN_SCU410_5,
    CLR_PIN_SCU4B0_5,
    PIN_SCU690_5,
    // CSOUT
    CLR_PIN_SCU410_6,
    CLR_PIN_SCU4B0_6,
    PIN_SCU690_6,
];
/// SPIM2
pub const PINCTRL_SPIM2_DEFAULT: &[PinctrlPin] = &[
    // CSIN
    CLR_PIN_SCU410_14,
    CLR_PIN_SCU4B0_14,
    PIN_SCU690_14,
    // CLKIN
    CLR_PIN_SCU410_15,
    CLR_PIN_SCU4B0_15,
    PIN_SCU690_15,
    // MOSIIN
    CLR_PIN_SCU410_16,
    CLR_PIN_SCU4B0_16,
    PIN_SCU690_16,
    // MISOIN
    CLR_PIN_SCU410_17,
    CLR_PIN_SCU4B0_17,
    PIN_SCU690_17,
    // IO2IN
    CLR_PIN_SCU410_18,
    CLR_PIN_SCU4B0_18,
    PIN_SCU690_18,
    // IO3IN
    CLR_PIN_SCU410_19,
    CLR_PIN_SCU4B0_19,
    PIN_SCU690_19,
    // CSOUT
    CLR_PIN_SCU410_20,
    CLR_PIN_SCU4B0_20,
    PIN_SCU690_20,
];
/// SPIM3
pub const PINCTRL_SPIM3_DEFAULT: &[PinctrlPin] = &[
    // CSIN
    CLR_PIN_SCU410_28,
    CLR_PIN_SCU4B0_28,
    PIN_SCU690_28,
    // CLKIN
    CLR_PIN_SCU410_29,
    CLR_PIN_SCU4B0_29,
    PIN_SCU690_29,
    // MOSIIN
    CLR_PIN_SCU410_30,
    CLR_PIN_SCU4B0_30,
    PIN_SCU690_30,
    // MISOIN
    CLR_PIN_SCU410_31,
    CLR_PIN_SCU4B0_31,
    PIN_SCU690_31,
    // IO2IN
    CLR_PIN_SCU414_0,
    CLR_PIN_SCU4B4_0,
    PIN_SCU694_0,
    // IO3IN
    CLR_PIN_SCU414_1,
    CLR_PIN_SCU4B4_1,
    PIN_SCU694_1,
    // CSOUT
    CLR_PIN_SCU414_2,
    CLR_PIN_SCU4B4_2,
    PIN_SCU694_2,
    // CLKOUT
    CLR_PIN_SCU414_3,
    CLR_PIN_SCU4B4_3,
    PIN_SCU694_3,
    // MOSIOUT
    CLR_PIN_SCU414_4,
    CLR_PIN_SCU4B4_4,
    PIN_SCU694_4,
    // MISOOUT
    CLR_PIN_SCU414_5,
    CLR_PIN_SCU4B4_5,
    PIN_SCU694_5,
    // IO2OUT
    CLR_PIN_SCU414_6,
    CLR_PIN_SCU4B4_6,
    PIN_SCU694_6,
    // IO3OUT
    CLR_PIN_SCU414_7,
    CLR_PIN_SCU4B4_7,
    PIN_SCU694_7,
];
/// SPIM4
pub const PINCTRL_SPIM4_DEFAULT: &[PinctrlPin] = &[
    // CSIN
    CLR_PIN_SCU414_10,
    CLR_PIN_SCU4B4_10,
    PIN_SCU694_10,
    // CLKIN
    CLR_PIN_SCU414_11,
    CLR_PIN_SCU4B4_11,
    PIN_SCU694_11,
    // MOSIIN
    CLR_PIN_SCU414_12,
    CLR_PIN_SCU4B4_12,
    PIN_SCU694_12,
    // MISOIN
    CLR_PIN_SCU414_13,
    CLR_PIN_SCU4B4_13,
    PIN_SCU694_13,
    // IO2IN
    CLR_PIN_SCU414_14,
    CLR_PIN_SCU4B4_14,
    PIN_SCU694_14,
    // IO3IN
    CLR_PIN_SCU414_15,
    CLR_PIN_SCU4B4_15,
    PIN_SCU694_15,
    // CSOUT
    CLR_PIN_SCU414_16,
    CLR_PIN_SCU4B4_16,
    PIN_SCU694_16,
    // CLKOUT
    CLR_PIN_SCU414_17,
    CLR_PIN_SCU4B4_17,
    PIN_SCU694_17,
    // MOSIOUT
    CLR_PIN_SCU414_18,
    CLR_PIN_SCU4B4_18,
    PIN_SCU694_18,
    // MISOOUT
    CLR_PIN_SCU414_19,
    CLR_PIN_SCU4B4_19,
    PIN_SCU694_19,
    // IO2OUT
    CLR_PIN_SCU414_20,
    CLR_PIN_SCU4B4_20,
    PIN_SCU694_20,
    // IO3OUT
    CLR_PIN_SCU414_21,
    CLR_PIN_SCU4B4_21,
    PIN_SCU694_21,
];
*/
// =============================================================================
// I3C pin groups
// =============================================================================
//
// The AST1060 routes each I3C bus to either a Low-Voltage (LV) pad set,
// enabled in SCU418, or a High-Voltage (HV) pad set, enabled in SCU4B8
// (SVD fields `EnblI3CSCLn/SDAn{LV,HV}FnPin`). Setting the bit selects the
// I3C function on that pad.
//
// Naming follows the PAC instance / `BUS_NUM`, 0-based: `PINCTRL_I3C0` is PAC
// `I3c` (bus 0), `_I3C1` is PAC `I3c1` (bus 1), etc. — matching the aspeed-rust
// reference's `PINCTRL_HVI3Cn` groups (e.g. `PINCTRL_HVI3C2` == bus 2 == PAC
// `I3c2`). NOTE the SVD names the pads 1-based (SDA1..SDA4); bus `n` uses the
// SVD's `SDA(n+1)`/`SCL(n+1)` pads.
//
// The HV groups additionally **clear** the conflicting LV function bits on the
// same pads (`CLR_PIN_SCU418_*`), exactly as the reference does — enabling the
// HV pad alone without first releasing the LV pad would leave both functions
// muxed onto it. LV groups need no such clear (HV defaults off).

/// I3C bus 0 (PAC `I3c`) — LV pads: SCL/SDA on SCU418[16:17].
pub const PINCTRL_I3C0: &[PinctrlPin] = &[PIN_SCU418_16, PIN_SCU418_17];
/// I3C bus 1 (PAC `I3c1`) — LV pads: SCL/SDA on SCU418[18:19].
pub const PINCTRL_I3C1: &[PinctrlPin] = &[PIN_SCU418_18, PIN_SCU418_19];
/// I3C bus 2 (PAC `I3c2`) — LV pads: SCL/SDA on SCU418[20:21].
pub const PINCTRL_I3C2: &[PinctrlPin] = &[PIN_SCU418_20, PIN_SCU418_21];
/// I3C bus 3 (PAC `I3c3`) — LV pads: SCL/SDA on SCU418[22:23].
pub const PINCTRL_I3C3: &[PinctrlPin] = &[PIN_SCU418_22, PIN_SCU418_23];

//AST1040/ast1080
/// I3C HCI bus 0 (PAC `I3c`) — LV pads: SCL on SCU400[00:02]/SDA on SCU400[04:06]
pub const PINCTRL_I3C_HCI0: &[PinctrlPin] = &[PIN_SCU400_0, PIN_SCU400_4];
/// I3C HCI bus 1 (PAC `I3c1`) — LV pads: SCL on SCU400[08:10]/SDA on SCU400[12:14].
pub const PINCTRL_I3C_HCI1: &[PinctrlPin] = &[PIN_SCU400_8, PIN_SCU400_12];
/// I3C HCI bus 2 (PAC `I3c2`) — LV pads: SC Lon SCU400[16:18]/SDA on SCU400[20:22].
pub const PINCTRL_I3C_HCI2: &[PinctrlPin] = &[PIN_SCU400_16, PIN_SCU400_20];
/// I3C HCI bus 3 (PAC `I3c3`) — LV pads: SCL on SCU400[24:26]/SDA on SCU400[28:30].
pub const PINCTRL_I3C_HCI3: &[PinctrlPin] = &[PIN_SCU400_24, PIN_SCU400_28];

/// I3C bus 0 (PAC `I3c`) — HV pads: SCU4B8[8:9], clearing LV SCU418[8:9],[16:17].
// pub const PINCTRL_HVI3C0: &[PinctrlPin] = &[
//     CLR_PIN_SCU418_8,
//     CLR_PIN_SCU418_9,
//     CLR_PIN_SCU418_16,
//     CLR_PIN_SCU418_17,
//     PIN_SCU4B8_8,
//     PIN_SCU4B8_9,
// ];
/// I3C bus 1 (PAC `I3c1`) — HV pads: SCU4B8[10:11], clearing LV SCU418[10:11],[18:19].
// pub const PINCTRL_HVI3C1: &[PinctrlPin] = &[
//     CLR_PIN_SCU418_10,
//     CLR_PIN_SCU418_11,
//     CLR_PIN_SCU418_18,
//     CLR_PIN_SCU418_19,
//     PIN_SCU4B8_10,
//     PIN_SCU4B8_11,
// ];
/// I3C bus 2 (PAC `I3c2`) — HV pads: SCU4B8[12:13], clearing LV SCU418[12:13],[20:21].
/// This is the bus/pad set the AST1060 Test Harness wires for I3C, and the one
/// the aspeed-rust EVB tests (`PINCTRL_HVI3C2`) use.
// pub const PINCTRL_HVI3C2: &[PinctrlPin] = &[
//     CLR_PIN_SCU418_12,
//     CLR_PIN_SCU418_13,
//     CLR_PIN_SCU418_20,
//     CLR_PIN_SCU418_21,
//     PIN_SCU4B8_12,
//     PIN_SCU4B8_13,
// ];
/// I3C bus 3 (PAC `I3c3`) — HV pads: SCU4B8[14:15], clearing LV SCU418[14:15],[22:23].
// pub const PINCTRL_HVI3C3: &[PinctrlPin] = &[
//     CLR_PIN_SCU418_14,
//     CLR_PIN_SCU418_15,
//     CLR_PIN_SCU418_22,
//     CLR_PIN_SCU418_23,
//     PIN_SCU4B8_14,
//     PIN_SCU4B8_15,
// ];

/// SGPIOM (Serial GPIO Master) pin group: mux selection on SCU41C[8:11].
///
/// Enables the four SGPIOM serial pins:
///   - bit 8  `sgpmclk` — serial clock out
///   - bit 9  `sgpmld`  — load/latch out
///   - bit 10 `sgpmo`   — serial data out
///   - bit 11 `sgpmi`   — serial data in
///
/// Mux bits derived from the AST10x0 datasheet / Zephyr `ast10x0-pinctrl.dtsi`
/// (`pin_sgpmclk/ld/o/i` = `SIG_DESC_SET(0x41C, 8..11)`).
pub const PINCTRL_SGPIOM: &[PinctrlPin] =
    &[PIN_SCU41C_8, PIN_SCU41C_9, PIN_SCU41C_10, PIN_SCU41C_11];

/// Macro to safely modify a register bit (set or clear).
macro_rules! modify_reg {
    ($reg:expr, $bit:expr, $clear:expr) => {{
        let mut val: u32 = $reg.read().bits();
        if $clear {
            val &= !(1 << $bit);
        } else {
            val |= (1 << $bit);
        }
        $reg.write(|w| unsafe { w.bits(val) });
    }};
}

impl ScuRegisters {
    /// Apply a pinctrl group configuration.
    ///
    /// Iterates through pin descriptors and applies each to the corresponding
    /// SCU register offset and bit position.
    ///
    /// # Example
    /// ```no_run
    /// # use ast10x0_peripherals::scu_ast1080::{ScuRegisters, pinctrl};
    /// # unsafe {
    /// let scu = ScuRegisters::new_global();
    /// scu.apply_pinctrl_group(&[pinctrl::CLR_PIN_SCU41C_0]);
    /// # }
    /// ```
    pub fn apply_pinctrl_group(&self, pins: &[PinctrlPin]) {
        let regs = self.regs();
        for pin in pins {
            match pin.offset {
                0x400 => modify_reg!(regs.scu400(), pin.bit, pin.clear),
                0x404 => modify_reg!(regs.scu404(), pin.bit, pin.clear),
                0x408 => modify_reg!(regs.scu408(), pin.bit, pin.clear),
                0x40C => modify_reg!(regs.scu40c(), pin.bit, pin.clear),
                0x410 => modify_reg!(regs.scu410(), pin.bit, pin.clear),
                0x414 => modify_reg!(regs.scu414(), pin.bit, pin.clear),
                0x418 => modify_reg!(regs.scu418(), pin.bit, pin.clear),
                0x41C => modify_reg!(regs.scu41c(), pin.bit, pin.clear),
                0x430 => modify_reg!(regs.scu430(), pin.bit, pin.clear),
                0x434 => modify_reg!(regs.scu434(), pin.bit, pin.clear),
                0x438 => modify_reg!(regs.scu438(), pin.bit, pin.clear),
                0x43c => modify_reg!(regs.scu43c(), pin.bit, pin.clear),
                0x440 => modify_reg!(regs.scu440(), pin.bit, pin.clear),
                0x444 => modify_reg!(regs.scu444(), pin.bit, pin.clear),
                0x448 => modify_reg!(regs.scu448(), pin.bit, pin.clear),
                0x44C => modify_reg!(regs.scu44c(), pin.bit, pin.clear),
                0x450 => modify_reg!(regs.scu450(), pin.bit, pin.clear),
                0x454 => modify_reg!(regs.scu454(), pin.bit, pin.clear),
                0x458 => modify_reg!(regs.scu458(), pin.bit, pin.clear),
                0x45C => modify_reg!(regs.scu45c(), pin.bit, pin.clear),
                0x460 => modify_reg!(regs.scu460(), pin.bit, pin.clear),
                /*
                0x690 => modify_reg!(regs.scu690(), pin.bit, pin.clear),
                0x694 => modify_reg!(regs.scu694(), pin.bit, pin.clear),
                0x698 => modify_reg!(regs.scu698(), pin.bit, pin.clear),
                0x69C => modify_reg!(regs.scu69c(), pin.bit, pin.clear),
                0x6B0 => modify_reg!(regs.scu6b0(), pin.bit, pin.clear),
                */
                _ => {} // Unknown offset, silently ignore
            }
        }
    }
}
