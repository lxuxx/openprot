# AST1080 GPIOA loopback test

Connect GPIOA6 (push-pull output) to GPIOA7 (input) with a jumper, with no
other drivers connected to those pins.

Build using the shared bare-metal runtime:

```bash
bazelisk build --platforms=//target/ast10x0 //target/ast10x0/tests/peripherals/gpio_ast1080/gpioa_loopback:gpioa_loopback
```

Outputs are `gpioa_loopback.elf` and `gpioa_loopback.bin` under
`bazel-bin/target/ast10x0/tests/peripherals/gpio_ast1080/gpioa_loopback/`.

Ported from `gpio/gpioi_loopback`, this test checks matching initial and
trigger input levels, no interrupt pending before the trigger, interrupt
status after the trigger, and clearing status for all six cases:
level-high, level-low, rising-edge, falling-edge, and both-edge in each
direction. Interrupt status is polled; interrupt-handler delivery is not tested.

Load the raw binary into SRAM at address zero and start through its vector
table. The loader must configure UART12/BMC-UART clocks, pinmux, baud rate,
and line format. Output is plain UART text ending in `TEST_RESULT:PASS` or
`TEST_RESULT:FAIL`. Building does not upload or run the firmware.
