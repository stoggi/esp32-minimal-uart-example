# Serial Interrupt Debug Tool

Minimal interrupt-based tool to debug USB Serial JTAG reading issues on ESP32-C3.

## Building

```bash
# Build and flash
cargo run --release

# Generate .bin file
espflash save-image --chip esp32c3 --merge \
  target/riscv32imc-unknown-none-elf/release/serial-async-debug \
  serial-async-debug.bin
```

## Output

Type "test" and press Enter:

```
[DEBUG] Read byte: 0x74 (t)
[DEBUG] Added to buffer, position now: 1
[DEBUG] Read byte: 0x65 (e)
[DEBUG] Added to buffer, position now: 2
[DEBUG] Read byte: 0x73 (s)
[DEBUG] Added to buffer, position now: 3
[DEBUG] Read byte: 0x74 (t)
[DEBUG] Added to buffer, position now: 4
[DEBUG] Read byte: 0x0D ()

[DEBUG] Got newline! Buffer contents (4 bytes):
[DEBUG] Raw bytes: [116, 101, 115, 116]
[DEBUG] As string: "test"
```

Prints every byte received (hex + char), buffer position, and final string when Enter is pressed.
