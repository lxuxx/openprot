# AST1080 GPIOA smoke test

Build using the shared I3C HCI bare-metal runtime:

```bash
bazelisk build --platforms=//target/ast10x0 //target/ast10x0/tests/peripherals/gpio_ast1080/gpio_smoke:gpio_smoke
```

Outputs are `gpio_smoke.elf` and `gpio_smoke.bin` under
`bazel-bin/target/ast10x0/tests/peripherals/gpio_ast1080/gpio_smoke/`.

The checks are ported from `gpio/gpio_smoke` using the AST1080 GPIO driver:
GPIOA0/A1 input configuration and sampling, GPIOA3 active-low open-drain
low/high state, and GPIOA4 push-pull low/high state. Input samples may be
either level. Keep GPIOA3/A4 free of conflicting external drivers.

Load the raw binary into SRAM at address zero and start through its vector
table. The loader must configure UART12/BMC-UART clocks, pinmux, baud rate,
and line format. Output is plain UART text ending in `TEST_RESULT:PASS` or
`TEST_RESULT:FAIL`. Building does not upload or run the firmware.
