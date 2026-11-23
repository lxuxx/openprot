// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! RAII teardown for in-flight master DMA transactions.
//!
//! A master DMA transfer arms the engine as an AHB bus master pointed at a
//! buffer in shared `.ram_nc`. If the transfer times out, returning an error
//! without stopping the engine leaves it free to keep writing into that buffer
//! (issue #359). This port retains the base driver
//! controller soft-reset teardown.
//!
//! [`ArmedDma`] makes that teardown a property of the type system: constructing
//! it arms the engine, and its [`Drop`] soft-resets the controller
//! unconditionally. [`ArmedDma::commit`] *consumes* the guard (defusing the
//! teardown by forgetting it), so a committed transaction is no longer a
//! droppable `ArmedDma` — "committed" is the absence of the value, not a runtime
//! flag. Every error path that returns before commit therefore drops a live
//! guard and tears the engine down; there is no state in which the teardown can
//! be skipped by mistake. All DMA-lifecycle `unsafe` is confined to this module,
//! holding its own `Copy` of the [`Ast1080I2cRegisters`] façade.

use super::constants;
use super::registers::Ast1080I2cRegisters;

/// Guard for one armed master DMA transaction.
///
/// Constructing an `ArmedDma` programs the DMA length + buffer-base registers
/// (the engine is now a potential AHB bus master). [`Drop`] soft-resets the
/// controller and waits for the engine to go idle. The happy path calls
/// [`commit`](ArmedDma::commit), which consumes the guard so its `Drop` never
/// runs — there is no "committed" flag, the committed state is simply the guard
/// no longer existing.
#[must_use = "drop tears down the DMA engine; bind it for the transfer's lifetime"]
pub(crate) struct ArmedDma {
    mmio: Ast1080I2cRegisters,
}

impl ArmedDma {
    // The PAC groups each length and its write-enable bit into one u16 field.
    // Retain the base driver's bit-15 length-write enable.
    /// Arm a TX DMA transaction: program i2c1c (len-1) + i2c30 (base addr).
    pub(crate) fn arm_tx(mmio: Ast1080I2cRegisters, phy_addr: u32, len: usize) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        mmio.i2c()
            .i2c1c()
            .write(|w| unsafe { w.mtxlen().bits((len - 1) as u16 | 0x8000) });
        mmio.i2c()
            .i2c30()
            .write(|w| unsafe { w.mtxa().bits(phy_addr) });
        Self { mmio }
    }

    /// Arm an RX DMA transaction: program i2c1c (len-1) + i2c34 (base addr).
    pub(crate) fn arm_rx(mmio: Ast1080I2cRegisters, phy_addr: u32, len: usize) -> Self {
        #[allow(clippy::cast_possible_truncation)]
        mmio.i2c()
            .i2c1c()
            .modify(|_, w| unsafe { w.mrxlen().bits((len - 1) as u16 | 0x8000) });
        mmio.i2c()
            .i2c34()
            .modify(|_, w| unsafe { w.mrxa().bits(phy_addr) });
        Self { mmio }
    }

    /// Transfer completed cleanly (STOP issued); no teardown needed.
    pub(crate) fn commit(self) {
        core::mem::forget(self);
    }
}

impl Drop for ArmedDma {
    fn drop(&mut self) {
        // Reached only for an *uncommitted* guard: `commit` consumes and forgets
        // the value, so a committed transaction never drops here. Teardown is
        // therefore unconditional.
        //
        // No master-only abort exists; soft-reset the controller (datasheet
        // §27.6.8): clear I2CC00 function-control, then restore it. Timing in
        // I2CC04 survives. Then spin until the engine reports idle, bounded so
        // a wedged controller cannot hang the drop.
        //
        // This disables the slave function for the reset window, which is safe
        // here: the i2c-server-runtime backend is master-only and never arms a
        // concurrent slave on this controller.
        let fun_ctrl = self.mmio.i2c().i2c00().read().bits();
        unsafe {
            self.mmio.i2c().i2c00().write(|w| w.bits(0));
            self.mmio.i2c().i2c00().write(|w| w.bits(fun_ctrl));
        }

        let mut timeout = constants::ABORT_TIMEOUT_US;
        while timeout > 0 && self.mmio.i2c().i2c08().read().bits() & (1 << 16) != 0 {
            timeout = timeout.saturating_sub(1);
            core::hint::spin_loop();
        }

        // Clear any latched interrupts from the aborted transaction.
        unsafe {
            self.mmio.i2c().i2c14().write(|w| w.bits(0xffff_ffff));
        }
    }
}
