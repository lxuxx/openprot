// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

//! AST1080 GPIOA loopback test target.

#![no_std]
#![no_main]

use ast10x0_gpio_ast1080::{GpioExt, InterruptMode, gpioa};
use ast10x0_scu_ast1080_pinctrl::{ScuRegisters, pinctrl};
use bare_metal::write_all as console_backend_write_all;
use embedded_hal::digital::{InputPin, OutputPin};

const GPIO_LEVEL_DELAY_CYCLES: u32 = 200_000_000;

fn test_gpio_loopback() -> bool {
    // Connect GPIOA6 (output) to GPIOA7 (input) with a jumper.
    // SAFETY: this standalone test is the sole owner of the SCU and GPIO peripherals.
    let gpioa = unsafe {
        let scu = ScuRegisters::new_global_unlocked();
        scu.apply_pinctrl_group(pinctrl::PINCTRL_GPIOA6);
        scu.apply_pinctrl_group(pinctrl::PINCTRL_GPIOA7);
        gpioa::GPIOA::new_global().split()
    };
    bare_metal::info!("--- GPIOA loopback test ---");

    let mut output = gpioa.pa6.into_push_pull_output();
    let mut input = gpioa.pa7.into_pull_down_input();

    let interrupt_cases = [
        ("level-high", InterruptMode::LevelHigh, false, true),
        ("level-low", InterruptMode::LevelLow, true, false),
        ("rising-edge", InterruptMode::EdgeRising, false, true),
        ("falling-edge", InterruptMode::EdgeFalling, true, false),
        ("both-edge rising", InterruptMode::EdgeBoth, false, true),
        ("both-edge falling", InterruptMode::EdgeBoth, true, false),
    ];

    for (name, mode, initial_high, trigger_high) in interrupt_cases {
        bare_metal::info!("=== Testing GPIOA7 {} interrupt ===", name as &str);

        input.set_interrupt_mode(InterruptMode::Disabled);
        input.clear_interrupt();

        let initial_result = if initial_high {
            output.set_high()
        } else {
            output.set_low()
        };
        if initial_result.is_err() {
            bare_metal::error!("{}: GPIOA6 could not drive initial level", name as &str);
            return false;
        }
        let initial_level = if initial_high { "high" } else { "low" };
        bare_metal::info!(
            "{}: GPIOA6 as output drive initial level {}",
            name as &str,
            initial_level as &str
        );
        cortex_m::asm::delay(GPIO_LEVEL_DELAY_CYCLES);

        let input_matches = if initial_high {
            input.is_high().unwrap_or(false)
        } else {
            input.is_low().unwrap_or(false)
        };
        if !input_matches {
            bare_metal::error!("{}: input and output GPIO level mismatch", name as &str);
            return false;
        }
        bare_metal::info!(
            "{}: GPIOA7 as input read initial level {}",
            name as &str,
            initial_level as &str
        );

        input.clear_interrupt();
        input.set_interrupt_mode(mode);
        if input.get_interrupt_status() {
            bare_metal::error!("{}: interrupt pending before trigger", name as &str);
            input.set_interrupt_mode(InterruptMode::Disabled);
            return false;
        }

        let trigger_result = if trigger_high {
            output.set_high()
        } else {
            output.set_low()
        };
        if trigger_result.is_err() {
            bare_metal::error!("{}: GPIOA6 could not drive trigger level", name as &str);
            input.set_interrupt_mode(InterruptMode::Disabled);
            return false;
        }
        let trigger_level = if trigger_high { "high" } else { "low" };
        bare_metal::info!(
            "{}: GPIOA6 as output drive trigger level {}",
            name as &str,
            trigger_level as &str
        );
        cortex_m::asm::delay(GPIO_LEVEL_DELAY_CYCLES);

        let input_matches = if trigger_high {
            input.is_high().unwrap_or(false)
        } else {
            input.is_low().unwrap_or(false)
        };
        if !input_matches {
            bare_metal::error!("{}: input and output GPIO level mismatch", name as &str);
            input.set_interrupt_mode(InterruptMode::Disabled);
            return false;
        }
        bare_metal::info!(
            "{}: GPIOA7 as input read trigger level {}",
            name as &str,
            trigger_level as &str
        );

        if !input.get_interrupt_status() {
            bare_metal::error!("{}: interrupt status was not set", name as &str);
            input.set_interrupt_mode(InterruptMode::Disabled);
            return false;
        }
        bare_metal::info!("GPIOA7 {} interrupt status set", name as &str);

        // Disable level-sensitive modes before clearing so an active level
        // cannot immediately reassert the status bit.
        input.set_interrupt_mode(InterruptMode::Disabled);
        input.clear_interrupt();
        if input.get_interrupt_status() {
            bare_metal::error!("{}: interrupt status did not clear", name as &str);
            return false;
        }
    }

    true
}

fn run_gpioa_loopback_test() -> bool {
    bare_metal::info!("=== AST1080 GPIOA loopback test ===");
    // Connect a jumper between GPIOA6 and GPIOA7 to verify the loopback.
    test_gpio_loopback()
}

#[unsafe(no_mangle)]
pub extern "C" fn test_main() -> ! {
    let sentinel = if run_gpioa_loopback_test() {
        b"TEST_RESULT:PASS\n"
    } else {
        b"TEST_RESULT:FAIL\n"
    };
    console_backend_write_all(sentinel);

    #[expect(clippy::empty_loop)]
    loop {}
}
