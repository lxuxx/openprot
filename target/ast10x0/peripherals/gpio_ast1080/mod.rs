// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! AST10x0 GPIO peripheral driver.
//!
//! Each GPIO bank (A–U) is exposed as a sub-module (`gpioa`, `gpiob`, …).
//! Obtain a bank via `GPIOX::new_global()` (unsafe), then call `split()` to
//! get the individual typed pins.

mod registers;
mod types;

pub use registers::GpioRegisters;
pub use types::{
    ActiveHigh, ActiveLow, Floating, GpioError, GpioExt, Input, InputMode, InterruptMode,
    OpenDrain, OpenDrainMode, Output, OutputMode, PullDown, PullUp, PushPull, Tristate,
};

macro_rules! gpio_macro {
    ($GPIOX:ident, $gpiox:ident, $x:literal, $pos:literal, [
            $($PXi:ident: ($pxi:ident, $i:literal, $ctrl_reg:ident, $MODE:ty),)+
        ]) => {

        pub mod $gpiox {
            use super::*;
            use ast1080_pac as device;
            use core::marker::PhantomData;
            use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin};

            pub struct $GPIOX {
                _regs: GpioRegisters,
            }

            impl $GPIOX {
                /// Create a GPIO bank instance from a [`GpioRegisters`] handle.
                ///
                /// # Safety
                ///
                /// Caller must ensure exclusive access to the GPIO register block is
                /// coordinated for the lifetime of this value.
                pub const unsafe fn new(regs: GpioRegisters) -> Self {
                    Self { _regs: regs }
                }

                /// Create a GPIO bank instance using the global GPIO register block.
                ///
                /// # Safety
                ///
                /// Caller must ensure exclusive access to the singleton GPIO peripheral
                /// is coordinated for the lifetime of this value.
                pub unsafe fn new_global() -> Self {
                    Self {
                        _regs: unsafe { GpioRegisters::new_global() },
                    }
                }

                /// Initialize command-source and debounce registers for this bank.
                pub fn init(&self) {
                    let p = self._regs.regs();
                    
                }

                pub fn set_debounce_timer(&self, timer1: u32, timer2: u32, timer3: u32) {
                    let p = self._regs.regs();

                    p.gpio000().write(|w| unsafe { w.bits(timer1) });
                    p.gpio004().write(|w| unsafe { w.bits(timer2) });
                    p.gpio008().write(|w| unsafe { w.bits(timer3) });
                }

                pub fn gpio_blink_src_clk_div(&self, div: u32) {
                    let p = self._regs.regs();

                    p.gpio00c().write(|w| unsafe { w.bits(div) });
                }

                pub fn gpio_blink_ctrl(&self, ctrl: u32) {
                    let p = self._regs.regs();

                    p.gpio010().write(|w| unsafe { w.bits(ctrl) });
                }
            }

            pub struct Parts {
                $(
                    pub $pxi: $PXi<$MODE>,
                )+
            }

            impl GpioExt for $GPIOX {
                type Parts = Parts;

                fn split(self) -> Self::Parts {
                    Parts {
                        $(
                            $pxi: $PXi {
                                _mode: PhantomData,
                            },
                        )+
                    }
                }
            }

            $(
                pub struct $PXi<MODE> {
                    _mode: PhantomData<MODE>,
                }

                impl<MODE> $PXi<MODE> {
                    /// Configures the pin as a pulled-down input.
                    #[must_use]
                    pub fn into_pull_down_input(self) -> $PXi<Input<PullDown>> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.direction_reg().set_bit()
                        });
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.data_reg().clear_bit()
                        });
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin as a pulled-up input.
                    #[must_use]
                    pub fn into_pull_up_input(self) -> $PXi<Input<PullUp>> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.direction_reg().set_bit()
                        });
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.data_reg().set_bit()
                        });
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin as an open-drain output.
                    #[must_use]
                    pub fn into_open_drain_output<ODM>(self) -> $PXi<Output<OpenDrain<ODM>>>
                    where
                        ODM: OpenDrainMode,
                    {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|r, w| unsafe {
                            w.data_reg(if ODM::ACTIVE_HIGH {
                                true
                            } else {
                                false
                            })
                        });
                        // Start released in the inactive state.
                        p.$ctrl_reg().modify(|r, w| unsafe {
                            w.direction_reg().clear_bit()
                        });
                        $PXi { _mode: PhantomData }
                    }

                    /// Configures the pin as a push-pull output.
                    #[must_use]
                    pub fn into_push_pull_output(self) -> $PXi<Output<PushPull>> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.direction_reg().set_bit()
                        });
                        p.$ctrl_reg().modify(|r, w| unsafe {
                            w.data_reg().set_bit()
                        });
                        $PXi { _mode: PhantomData }
                    }
                }

                impl StatefulOutputPin for $PXi<Output<PushPull>> {
                    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        Ok(
                            p.$ctrl_reg().read().input_data_reg().bit(),
                        )
                    }

                    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                        self.is_set_high().map(|v| !v)
                    }
                }

                impl OutputPin for $PXi<Output<PushPull>> {
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.data_reg().set_bit()
                        });
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|_, w| unsafe {
                            w.data_reg().clear_bit()
                        });
                        Ok(())
                    }
                }

                impl<ODM> StatefulOutputPin for $PXi<Output<OpenDrain<ODM>>>
                where
                    ODM: OpenDrainMode,
                {
                    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        let is_output =
                            (p.$ctrl_reg().read().direction_reg().bit()) != 0;
                        Ok(is_output == ODM::ACTIVE_HIGH)
                    }

                    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
                        self.is_set_high().map(|v| !v)
                    }
                }

                impl<ODM> OutputPin for $PXi<Output<OpenDrain<ODM>>>
                where
                    ODM: OpenDrainMode,
                {
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|r, w| unsafe {
                            if ODM::ACTIVE_HIGH {
                                w.direction_reg().set_bit()
                            } else {
                                w.direction_reg().clear_bit()
                            }
                        });
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().modify(|r, w| unsafe {
                            if ODM::ACTIVE_HIGH {
                                w.direction_reg().clear_bit()
                            } else {
                                w.direction_reg().set_bit()
                            }
                        });
                        Ok(())
                    }
                }

                impl<MODE> InputPin for $PXi<Input<MODE>>
                where
                    MODE: InputMode,
                {
                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        let p = unsafe { &*device::Gpio::ptr() };
                        Ok(
                            (p.$ctrl_reg().read().input_data_reg().bit() != 0),
                        )
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        self.is_high().map(|v| !v)
                    }
                }

                impl<MODE> $PXi<Input<MODE>>
                where
                    MODE: InputMode,
                {
                    /// Enable or disable interrupts on this pin.
                    pub fn set_interrupt_mode(&mut self, mode: InterruptMode) {
                        let p = unsafe { &*device::Gpio::ptr() };
                        match mode {
                            InterruptMode::LevelHigh => {
                                p.$ctrl_reg().write().intsensitivity_type0sel().set_bit();
                                p.$ctrl_reg().write().intsensitivity_type1sel().set_bit();
                                p.$ctrl_reg().write().intsensitivity_type2sel().clear_bit();
                                p.$ctrl_reg().write().intenbl().set_bit();
                            }
                            InterruptMode::LevelLow => {
                                p.$ctrl_reg().write().intsensitivity_type0sel().clear_bit();
                                p.$ctrl_reg().write().intsensitivity_type1sel().set_bit();
                                p.$ctrl_reg().write().intsensitivity_type2sel().clear_bit();
                                p.$ctrl_reg().write().intenbl().set_bit();
                            }
                            InterruptMode::EdgeRising => {
                                p.$ctrl_reg().write().intsensitivity_type0sel().set_bit();
                                p.$ctrl_reg().write().intsensitivity_type1sel().clear_bit();
                                p.$ctrl_reg().write().intsensitivity_type2sel().clear_bit();
                                p.$ctrl_reg().write().intenbl().set_bit();
                            }
                            InterruptMode::EdgeFalling => {
                                p.$ctrl_reg().write().intsensitivity_type0sel().clear_bit();
                                p.$ctrl_reg().write().intsensitivity_type1sel().clear_bit();
                                p.$ctrl_reg().write().intsensitivity_type2sel().clear_bit();
                                p.$ctrl_reg().write().intenbl().set_bit();
                            }
                            InterruptMode::EdgeBoth => {
                                p.$ctrl_reg().write().intsensitivity_type2sel().set_bit();
                                p.$ctrl_reg().write().intenbl().set_bit();
                            }
                            InterruptMode::Disabled => {
                                p.$ctrl_reg().write().intenbl().clear_bit();
                            }
                        }
                    }

                    /// Returns the current interrupt-pending status for this pin.
                    #[must_use]
                    pub fn get_interrupt_status(&self) -> bool {
                        let p = unsafe { &*device::Gpio::ptr() };

                        (p.$ctrl_reg().read().intsts_reg().bit()
                            == 1)
                    }

                    /// Clear the pending interrupt for this pin.
                    pub fn clear_interrupt(&self) {
                        let p = unsafe { &*device::Gpio::ptr() };
                        
                        p.$ctrl_reg().write().intsts_reg().set_bit();
                    }

                    /// Set the command-source bits for this pin.
                    pub fn set_cmd_src(&self, cmd_src0: u32, cmd_src1: u32) {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$cmd_src0_reg().modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(1u32 << ($pos + $i)))
                                    | (cmd_src0 << ($pos + $i)),
                            )
                        });
                        p.$cmd_src1_reg().modify(|r, w| unsafe {
                            w.bits(
                                (r.bits() & !(1u32 << ($pos + $i)))
                                    | (cmd_src1 << ($pos + $i)),
                            )
                        });
                    }

                    /// Select the debounce timer for this pin.
                    pub fn select_debounce_timer(&self, deb_setting1: u32, deb_setting2: u32) {
                        let p = unsafe { &*device::Gpio::ptr() };
                        p.$ctrl_reg().write().deb1_reg().bit(deb_setting1 != 0);
                        p.$ctrl_reg().write().deb2_reg().bit(deb_setting2 != 0);
                    }
                }

                impl<MODE> embedded_hal::digital::ErrorType for $PXi<MODE> {
                    type Error = GpioError;
                }
            )+
        }
    };
}

// GPIO ABCD — register group 0x000–0x01c
gpio_macro!(
    GPIOA, gpioa, 'a', 0,
    [
        PA0: (pa0, 0, gpio180, Tristate),
        PA1: (pa1, 1, gpio184, Tristate),
        PA2: (pa2, 2, gpio188, Tristate),
        PA3: (pa3, 3, gpio18c, Tristate),
        PA4: (pa4, 4, gpio190, Tristate),
        PA5: (pa5, 5, gpio194, Tristate),
        PA6: (pa6, 6, gpio198, Tristate),
        PA7: (pa7, 7, gpio19c, Tristate),
    ]
);

gpio_macro!(
    GPIOB, gpiob, 'b', 8,
    [
        PB0: (pb0, 0, gpio1a0, Tristate),
        PB1: (pb1, 1, gpio1a4, Tristate),
        PB2: (pb2, 2, gpio1a8, Tristate),
        PB3: (pb3, 3, gpio1ac, Tristate),
        PB4: (pb4, 4, gpio1b0, Tristate),
        PB5: (pb5, 5, gpio1b4, Tristate),
        PB6: (pb6, 6, gpio1b8, Tristate),
        PB7: (pb7, 7, gpio1bc, Tristate),
    ]
);

gpio_macro!(
    GPIOC, gpioc, 'c', 16,
    [
        PC0: (pc0, 0, gpio1c0, Tristate),
        PC1: (pc1, 1, gpio1c4, Tristate),
        PC2: (pc2, 2, gpio1c8, Tristate),
        PC3: (pc3, 3, gpio1cc, Tristate),
        PC4: (pc4, 4, gpio1d0, Tristate),
        PC5: (pc5, 5, gpio1d4, Tristate),
        PC6: (pc6, 6, gpio1d8, Tristate),
        PC7: (pc7, 7, gpio1dc, Tristate),
    ]
);

gpio_macro!(
    GPIOD, gpiod, 'd', 24,
    [
        PD0: (pd0, 0, gpio1e0, Tristate),
        PD1: (pd1, 1, gpio1e4, Tristate),
        PD2: (pd2, 2, gpio1e8, Tristate),
        PD3: (pd3, 3, gpio1ec, Tristate),
        PD4: (pd4, 4, gpio1f0, Tristate),
        PD5: (pd5, 5, gpio1f4, Tristate),
        PD6: (pd6, 6, gpio1f8, Tristate),
        PD7: (pd7, 7, gpio1fc, Tristate),
    ]
);

// GPIO EFGH — register group 0x020–0x03c
gpio_macro!(
    GPIOE, gpioe, 'e', 0,
    [
        PE0: (pe0, 0, gpio200, Tristate),
        PE1: (pe1, 1, gpio204, Tristate),
        PE2: (pe2, 2, gpio208, Tristate),
        PE3: (pe3, 3, gpio20c, Tristate),
        PE4: (pe4, 4, gpio210, Tristate),
        PE5: (pe5, 5, gpio214, Tristate),
        PE6: (pe6, 6, gpio218, Tristate),
        PE7: (pe7, 7, gpio21c, Tristate),
    ]
);

gpio_macro!(
    GPIOF, gpiof, 'f', 8,
    [
        PF0: (pf0, 0, gpio220, Tristate),
        PF1: (pf1, 1, gpio224, Tristate),
        PF2: (pf2, 2, gpio228, Tristate),
        PF3: (pf3, 3, gpio22c, Tristate),
        PF4: (pf4, 4, gpio230, Tristate),
        PF5: (pf5, 5, gpio234, Tristate),
        PF6: (pf6, 6, gpio238, Tristate),
        PF7: (pf7, 7, gpio23c, Tristate),
    ]
);

gpio_macro!(
    GPIOG, gpiog, 'g', 16,
    [
        PG0: (pg0, 0, gpio240, Tristate),
        PG1: (pg1, 1, gpio244, Tristate),
        PG2: (pg2, 2, gpio248, Tristate),
        PG3: (pg3, 3, gpio24c, Tristate),
        PG4: (pg4, 4, gpio250, Tristate),
        PG5: (pg5, 5, gpio254, Tristate),
        PG6: (pg6, 6, gpio258, Tristate),
        PG7: (pg7, 7, gpio25c, Tristate),
    ]
);

gpio_macro!(
    GPIOH, gpioh, 'h', 24,
    [
        PH0: (ph0, 0, gpio260, Tristate),
        PH1: (ph1, 1, gpio264, Tristate),
        PH2: (ph2, 2, gpio268, Tristate),
        PH3: (ph3, 3, gpio26c, Tristate),
        PH4: (ph4, 4, gpio270, Tristate),
        PH5: (ph5, 5, gpio274, Tristate),
        PH6: (ph6, 6, gpio278, Tristate),
        PH7: (ph7, 7, gpio27c, Tristate),
    ]
);

// GPIO IJKL — register group 0x070–0x0b8
gpio_macro!(
    GPIOI, gpioi, 'i', 0,
    [
        PI0: (pi0, 0, gpio280, Tristate),
        PI1: (pi1, 1, gpio284, Tristate),
        PI2: (pi2, 2, gpio288, Tristate),
        PI3: (pi3, 3, gpio28c, Tristate),
        PI4: (pi4, 4, gpio290, Tristate),
        PI5: (pi5, 5, gpio294, Tristate),
        PI6: (pi6, 6, gpio298, Tristate),
        PI7: (pi7, 7, gpio29c, Tristate),
    ]
);

gpio_macro!(
    GPIOJ, gpioj, 'j', 8,
    [
        PJ0: (pj0, 0, gpio2a0, Tristate),
        PJ1: (pj1, 1, gpio2a4, Tristate),
        PJ2: (pj2, 2, gpio2a8, Tristate),
        PJ3: (pj3, 3, gpio2ac, Tristate),
        PJ4: (pj4, 4, gpio2b0, Tristate),
        PJ5: (pj5, 5, gpio2b4, Tristate),
        PJ6: (pj6, 6, gpio2b8, Tristate),
        PJ7: (pj7, 7, gpio2bc, Tristate),
    ]
);

gpio_macro!(
    GPIOK, gpiok, 'k', 16,
    [
        PK0: (pk0, 0, gpio2c0, Tristate),
        PK1: (pk1, 1, gpio2c4, Tristate),
        PK2: (pk2, 2, gpio2c8, Tristate),
        PK3: (pk3, 3, gpio2cc, Tristate),
        PK4: (pk4, 4, gpio2d0, Tristate),
        PK5: (pk5, 5, gpio2d4, Tristate),
        PK6: (pk6, 6, gpio2d8, Tristate),
        PK7: (pk7, 7, gpio2dc, Tristate),
    ]
);

gpio_macro!(
    GPIOL, gpiol, 'l', 24,
    [
        PL0: (pl0, 0, gpio2e0, Tristate),
        PL1: (pl1, 1, gpio2e4, Tristate),
        PL2: (pl2, 2, gpio2e8, Tristate),
        PL3: (pl3, 3, gpio2ec, Tristate),
        PL4: (pl4, 4, gpio2f0, Tristate),
        PL5: (pl5, 5, gpio2f4, Tristate),
        PL6: (pl6, 6, gpio2f8, Tristate),
        PL7: (pl7, 7, gpio2fc, Tristate),
    ]
);

// GPIO MNOP — register group 0x078–0x108
gpio_macro!(
    GPIOM, gpiom, 'm', 0,
    [
        PM0: (pm0, 0, gpio300, Tristate),
        PM1: (pm1, 1, gpio304, Tristate),
        PM2: (pm2, 2, gpio308, Tristate),
        PM3: (pm3, 3, gpio30c, Tristate),
        PM4: (pm4, 4, gpio310, Tristate),
        PM5: (pm5, 5, gpio314, Tristate),
        PM6: (pm6, 6, gpio318, Tristate),
        PM7: (pm7, 7, gpio31c, Tristate),
    ]
);

gpio_macro!(
    GPION, gpion, 'n', 8,
    [
        PN0: (pn0, 0, gpio320, Tristate),
        PN1: (pn1, 1, gpio324, Tristate),
        PN2: (pn2, 2, gpio328, Tristate),
        PN3: (pn3, 3, gpio32c, Tristate),
        PN4: (pn4, 4, gpio330, Tristate),
        PN5: (pn5, 5, gpio334, Tristate),
        PN6: (pn6, 6, gpio338, Tristate),
        PN7: (pn7, 7, gpio33c, Tristate),
    ]
);

gpio_macro!(
    GPIOO, gpioo, 'o', 16,
    [
        PO0: (po0, 0, gpio340, Tristate),
        PO1: (po1, 1, gpio344, Tristate),
        PO2: (po2, 2, gpio348, Tristate),
        PO3: (po3, 3, gpio34c, Tristate),
        PO4: (po4, 4, gpio350, Tristate),
        PO5: (po5, 5, gpio354, Tristate),
        PO6: (po6, 6, gpio358, Tristate),
        PO7: (po7, 7, gpio35c, Tristate),
    ]
);

gpio_macro!(
    GPIOP, gpiop, 'p', 24,
    [
        PP0: (pp0, 0, gpio360, Tristate),
        PP1: (pp1, 1, gpio364, Tristate),
        PP2: (pp2, 2, gpio368, Tristate),
        PP3: (pp3, 3, gpio36c, Tristate),
        PP4: (pp4, 4, gpio370, Tristate),
        PP5: (pp5, 5, gpio374, Tristate),
        PP6: (pp6, 6, gpio378, Tristate),
        PP7: (pp7, 7, gpio37c, Tristate),
    ]
);

// GPIO QRST — register group 0x080–0x138
gpio_macro!(
    GPIOQ, gpioq, 'q', 0,
    [
        PQ0: (pq0, 0, gpio380, Tristate),
        PQ1: (pq1, 1, gpio384, Tristate),
        PQ2: (pq2, 2, gpio388, Tristate),
        PQ3: (pq3, 3, gpio38c, Tristate),
        PQ4: (pq4, 4, gpio390, Tristate),
        PQ5: (pq5, 5, gpio394, Tristate),
        PQ6: (pq6, 6, gpio398, Tristate),
        PQ7: (pq7, 7, gpio39c, Tristate),
    ]
);

gpio_macro!(
    GPIOR, gpior, 'r', 8,
    [
        PR0: (pr0, 0, gpio3a0, Tristate),
        PR1: (pr1, 1, gpio3a4, Tristate),
        PR2: (pr2, 2, gpio3a8, Tristate),
        PR3: (pr3, 3, gpio3ac, Tristate),
        PR4: (pr4, 4, gpio3b0, Tristate),
        PR5: (pr5, 5, gpio3b4, Tristate),
        PR6: (pr6, 6, gpio3b8, Tristate),
        PR7: (pr7, 7, gpio3bc, Tristate),
    ]
);

gpio_macro!(
    GPIOS, gpios, 's', 16,
    [
        PS0: (ps0, 0, gpio3c0, Tristate),
        PS1: (ps1, 1, gpio3c4, Tristate),
        PS2: (ps2, 2, gpio3c8, Tristate),
        PS3: (ps3, 3, gpio3cc, Tristate),
        PS4: (ps4, 4, gpio3d0, Tristate),
        PS5: (ps5, 5, gpio3d4, Tristate),
        PS6: (ps6, 6, gpio3d8, Tristate),
        PS7: (ps7, 7, gpio3dc, Tristate),
    ]
);

gpio_macro!(
    GPIOT, gpiot, 't', 24,
    [
        PT0: (pt0, 0, gpio3e0, Tristate),
        PT1: (pt1, 1, gpio3e4, Tristate),
        PT2: (pt2, 2, gpio3e8, Tristate),
        PT3: (pt3, 3, gpio3ec, Tristate),
        PT4: (pt4, 4, gpio3f0, Tristate),
        PT5: (pt5, 5, gpio3f4, Tristate),
        PT6: (pt6, 6, gpio3f8, Tristate),
        PT7: (pt7, 7, gpio3fc, Tristate),
    ]
);

// GPIO U — register group 0x088–0x168
gpio_macro!(
    GPIOU, gpiou, 'u', 0,
    [
        PU0: (pu0, 0, gpio400, Tristate),
        PU1: (pu1, 1, gpio404, Tristate),
        PU2: (pu2, 2, gpio408, Tristate),
        PU3: (pu3, 3, gpio40c, Tristate),
        PU4: (pu4, 4, gpio410, Tristate),
        PU5: (pu5, 5, gpio414, Tristate),
        PU6: (pu6, 6, gpio418, Tristate),
        PU7: (pu7, 7, gpio41c, Tristate),
    ]
);

gpio_macro!(
    GPIOV, gpiov, 'v', 8,
    [
        PV0: (pv0, 0, gpio420, Tristate),
        PV1: (pv1, 1, gpio424, Tristate),
        PV2: (pv2, 2, gpio428, Tristate),
        PV3: (pv3, 3, gpio42c, Tristate),
        PV4: (pv4, 4, gpio430, Tristate),
        PV5: (pv5, 5, gpio434, Tristate),
        PV6: (pv6, 6, gpio438, Tristate),
        PV7: (pv7, 7, gpio43c, Tristate),
    ]
);

gpio_macro!(
    GPIOW, gpiow, 'w', 16,
    [
        PW0: (pw0, 0, gpio440, Tristate),
        PW1: (pw1, 1, gpio444, Tristate),
        PW2: (pw2, 2, gpio448, Tristate),
        PW3: (pw3, 3, gpio44c, Tristate),
        PW4: (pw4, 4, gpio450, Tristate),
        PW5: (pw5, 5, gpio454, Tristate),
        PW6: (pw6, 6, gpio458, Tristate),
        PW7: (pw7, 7, gpio45c, Tristate),
    ]
);

gpio_macro!(
    GPIOX, gpiox, 'x', 24,
    [
        PX0: (px0, 0, gpio460, Tristate),
        PX1: (px1, 1, gpio464, Tristate),
        PX2: (px2, 2, gpio468, Tristate),
        PX3: (px3, 3, gpio46c, Tristate),
        PX4: (px4, 4, gpio470, Tristate),
        PX5: (px5, 5, gpio474, Tristate),
        PX6: (px6, 6, gpio478, Tristate),
        PX7: (px7, 7, gpio47c, Tristate),
    ]
);

gpio_macro!(
    GPIOY, gpioy, 'y', 32,
    [
        PY0: (py0, 0, gpio480, Tristate),
        PY1: (py1, 1, gpio484, Tristate),
    ]
);
