# AST1080 I2C initialization smoke test

Port of `i2c/i2c_init` using the AST1080 I2C driver and the shared
AST1080 bare-metal runtime.

## Build

```bash
bazelisk build --platforms=//target/ast10x0 //target/ast10x0/tests/peripherals/i2c_ast1080/i2c_init:i2c_init
```

Outputs are `i2c_init.elf` and `i2c_init.bin` under
`bazel-bin/target/ast10x0/tests/peripherals/i2c_ast1080/i2c_init/`.

## Coverage

The test selects `PINCTRL_I2C1`, resets the shared I2C block using
SCU220/224 bit 15, enables its clock using SCU240/244 bit 15, and calls
`init_i2c_global()` once. It verifies reset release, clock enable, global
control, and the `0x6222_0803` clock-divider configuration.

For Standard, Fast, Fast-plus, and DMA-fast modes, it constructs
`Ast1080I2c` and checks I2C00, I2C04, and I2C10 for master enable,
auto-release, multi-master configuration, timing, SMBus timeout, and
interrupt enables. Failures print register diagnostics.

The DMA case exercises `new_with_dma` only. No bus transfers or DMA commands
are issued; the local buffers are never given to an active DMA engine.

## Run on hardware

Load the raw binary into SRAM at address zero and start through its vector
table, matching the other AST1080 bare-metal tests. The loader must configure:

- A 50 MHz I2C APB clock matching the test's static timing expectations.
- UART12/BMC-UART clocks, pinmux, baud rate, and line format.

The test resets the shared I2C block, so all I2C buses must be idle and owned
by this firmware. No external I2C target is required.

UART output ends in `TEST_RESULT:PASS` or `TEST_RESULT:FAIL`. Building does
not upload or execute the firmware. This directory uses the shared SRAM
linker script and runtime rather than a kernel `system.json5`.
