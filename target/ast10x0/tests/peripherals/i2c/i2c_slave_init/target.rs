// Licensed under the Apache-2.0 license
// SPDX-License-Identifier: Apache-2.0

#![no_std]
#![no_main]

use ast10x0_board::{Ast10x0Board, Ast10x0BoardDescriptor};
use ast10x0_peripherals::i2c::{
    AST_I2CC_SLAVE_PKT_SAVE_ADDR, AST_I2CS_ACTIVE_ALL, AST_I2CS_INACTIVE_TO, AST_I2CS_PKT_DONE,
    AST_I2CS_PKT_MODE_EN, AST_I2CS_RX_BUFF_EN, Ast1060I2c, Ast1060I2cRegisters, ClockConfig,
    I2cConfig, I2cSpeed, I2cXferMode, SlaveConfig, SlaveEvent,
};
use ast10x0_peripherals::scu::pinctrl;
use codegen as _;
use console_backend::console_backend_write_all;
use entry as _;
use target_common::{TargetInterface, declare_target};

pub struct Target {}

const SLAVE_ADDR: u8 = 0x42;
const I2C_CONFIG: I2cConfig = I2cConfig {
    speed: I2cSpeed::Standard,
    xfer_mode: I2cXferMode::BufferMode,
    multi_master: false,
    smbus_timeout: true,
    smbus_alert: false,
    clock_config: ClockConfig::ast1060_default(),
};

fn dump_i2c2_registers() {
    // SAFETY: This test owns I2C2 for its lifetime.
    let regs = unsafe { &*ast1060_pac::I2c2::ptr() };
    pw_log::info!("--- I2C2 registers after slave init ---");
    macro_rules! dump {
        ($register:ident) => {
            pw_log::info!(
                "{}=0x{:08x}",
                stringify!($register) as &str,
                regs.$register().read().bits() as u32
            );
        };
    }
    dump!(i2cc00);
    dump!(i2cc04);
    dump!(i2cc08);
    dump!(i2cc0c);
    dump!(i2cm10);
    dump!(i2cm14);
    dump!(i2cm18);
    dump!(i2cm1c);
    dump!(i2cs20);
    dump!(i2cs24);
    dump!(i2cs28);
    dump!(i2cs2c);
    dump!(i2cm30);
    dump!(i2cm34);
    dump!(i2cs38);
    dump!(i2cs3c);
    dump!(i2cs40);
    dump!(i2cm48);
    dump!(i2cs4c);
    dump!(i2cc50);
    dump!(i2cc54);
}

fn verify_slave_registers() -> Result<(), &'static str> {
    // SAFETY: This test owns I2C2 for its lifetime.
    let regs = unsafe { &*ast1060_pac::I2c2::ptr() };
    let control = regs.i2cc00().read();
    let address = regs.i2cs40().read();
    let command = regs.i2cs28().read().bits();
    let interrupts = regs.i2cs20().read().bits();
    let expected_command = AST_I2CS_PKT_MODE_EN | AST_I2CS_ACTIVE_ALL | AST_I2CS_RX_BUFF_EN;
    let expected_interrupts = AST_I2CS_PKT_DONE | AST_I2CS_INACTIVE_TO;

    if !control.enbl_slave_fn().bit()
        || control.bits() & AST_I2CC_SLAVE_PKT_SAVE_ADDR == 0
        || address.slave_dev_addr1().bits() != SLAVE_ADDR
        || !address.enbl_slave_dev_addr1only_for_new_reg_mode().bit()
        || command & expected_command != expected_command
        || interrupts & expected_interrupts != expected_interrupts
    {
        return Err("slave register verification failed");
    }
    Ok(())
}

fn run_slave_init_test() -> Result<(), &'static str> {
    pw_log::info!("=== AST10x0 I2C2 slave init test ===");

    let board = Ast10x0Board::new(Ast10x0BoardDescriptor {
        pinctrl_groups: &[pinctrl::PINCTRL_I2C2],
        i2c_buses: &[],
    });
    // SAFETY: The test runs once at boot and owns the board peripherals.
    unsafe { board.init() }.map_err(|_| "board init failed")?;

    // SAFETY: The test owns I2C2 and its buffer for the process lifetime.
    let mut slave = unsafe {
        let mmio = Ast1060I2cRegisters::new(ast1060_pac::I2c2::ptr(), ast1060_pac::I2cbuff2::ptr());
        Ast1060I2c::new(mmio, &I2C_CONFIG, |_| core::hint::spin_loop())
    }
    .map_err(|_| "I2C2 init failed")?;

    let slave_config = SlaveConfig::new(SLAVE_ADDR).map_err(|_| "invalid slave address")?;
    slave
        .configure_slave(&slave_config)
        .map_err(|_| "configure_slave failed")?;

    dump_i2c2_registers();
    verify_slave_registers()?;
    pw_log::info!(
        "I2C2 slave ready at 0x{:02x}; waiting for master requests",
        SLAVE_ADDR as u32
    );

    const READ_RESPONSE: &[u8] = &[0x55];
    slave
        .slave_write(READ_RESPONSE)
        .map_err(|_| "arming slave read response failed")?;

    loop {
        match slave.handle_slave_interrupt() {
            Some(SlaveEvent::WriteRequest) => pw_log::info!("master write request"),
            Some(SlaveEvent::ReadRequest) => pw_log::info!("master read request"),
            Some(SlaveEvent::DataReceived { len }) => {
                pw_log::info!("received {} byte(s)", len as u32);
                let mut data = [0u8; 32];
                match slave.slave_read(&mut data) {
                    Ok(n) => {
                        for (index, byte) in data.iter().take(n).enumerate() {
                            pw_log::info!("rx[{}]=0x{:02x}", index as u32, *byte as u32);
                        }
                    }
                    Err(_) => pw_log::error!("slave_read failed"),
                }
            }
            Some(SlaveEvent::DataSent { len }) => {
                pw_log::info!("sent {} byte(s)", len as u32);
                if slave.slave_write(READ_RESPONSE).is_err() {
                    pw_log::error!("rearming slave read response failed");
                }
            }
            Some(SlaveEvent::DataReceivedAndSent { rx_len, tx_len }) => {
                pw_log::info!(
                    "received {} byte(s), sent {} byte(s)",
                    rx_len as u32,
                    tx_len as u32
                );
                let mut data = [0u8; 32];
                if let Ok(n) = slave.slave_read(&mut data) {
                    for (index, byte) in data.iter().take(n).enumerate() {
                        pw_log::info!("rx[{}]=0x{:02x}", index as u32, *byte as u32);
                    }
                }
                if slave.slave_write(READ_RESPONSE).is_err() {
                    pw_log::error!("rearming slave read response failed");
                }
            }
            Some(SlaveEvent::Stop) => pw_log::info!("master stop"),
            None => core::hint::spin_loop(),
        }
    }
}

impl TargetInterface for Target {
    const NAME: &'static str = "AST10x0 I2C Slave Init";

    fn main() -> ! {
        let sentinel: &[u8] = match run_slave_init_test() {
            Ok(()) => b"TEST_RESULT:PASS\n",
            Err(error) => {
                pw_log::error!("I2C slave init test failed: {}", error as &str);
                b"TEST_RESULT:FAIL\n"
            }
        };
        let _ = console_backend_write_all(sentinel);
        #[expect(clippy::empty_loop)]
        loop {}
    }
}

declare_target!(Target);
