# i2c_slave_init — AST10x0 I2C2 slave initialization

This hardware test configures I2C2 as a 0x42 slave using standard speed and
buffer mode, matching the slave setup in the linked `aspeed-rust` example.
It prints every defined I2C2 register immediately after `configure_slave`,
then waits for an external master. Incoming slave events and received bytes
are logged. A one-byte 0x55 response is armed for master reads.

The test deliberately remains active and does not emit `TEST_RESULT:PASS`.
Initialization or register verification failures emit `TEST_RESULT:FAIL`.

Build the image with:

```bash
bazel build --config=k_ast1060_evb \
  //target/ast10x0/tests/peripherals/i2c/i2c_slave_init:i2c_slave_init
```

Start the slave image before the external master sends requests to address
0x42. The hardware test target is available for interactive runs; its normal
timeout applies if the process is left waiting for requests.