// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! Configuration shared by every AST1080 GPIO bank.

use super::GpioRegisters;

/// Shared debounce and blink configuration for the entire GPIO peripheral.
/// Changing these registers affects every pin using the corresponding resource.
pub struct GpioController {
    regs: GpioRegisters,
}

impl GpioController {
    /// Construct a controller using a supplied register handle.
    ///
    /// # Safety
    /// The caller must coordinate access to the shared debounce and blink
    /// registers for the lifetime of this controller, including across banks.
    pub const unsafe fn new(regs: GpioRegisters) -> Self {
        Self { regs }
    }

    /// Construct a controller for the global GPIO peripheral.
    ///
    /// # Safety
    /// The same shared-register access contract as [`Self::new`] applies.
    pub unsafe fn new_global() -> Self {
        Self { regs: unsafe { GpioRegisters::new_global() } }
    }

    pub fn set_debounce_timer(&self, timer1: u32, timer2: u32, timer3: u32) {
        let p = self.regs.regs();

        p.gpio000().write(|w| unsafe { w.debounce_timer_value().bits(timer1) });
        p.gpio004().write(|w| unsafe { w.debounce_timer_value().bits(timer2) });
        p.gpio008().write(|w| unsafe { w.debounce_timer_value().bits(timer3) });
    }

    pub fn gpio_blink_src_clk_div(&self, div: u32) {
        let p = self.regs.regs();

        p.gpio00c().write(|w| unsafe {
            w.blink_source_clk_division_only_effective_to_pclkas_src_clk().bits(div)
        });
    }

    pub fn gpio_blink_ctrl(&self, ctrl: u32) {
        let p = self.regs.regs();

        p.gpio010().write(|w| unsafe { w.bits(ctrl & 0x333) });
    }

    pub fn gpio_blink_counter_config(&self, counter1_config: u32, counter2_config: u32, counter3_config: u32) {
        let p = self.regs.regs();

        p.gpio014().write(|w| unsafe { w.bits(counter1_config) });
        p.gpio018().write(|w| unsafe { w.bits(counter2_config) });
        p.gpio01c().write(|w| unsafe { w.bits(counter3_config) });
    }
}
