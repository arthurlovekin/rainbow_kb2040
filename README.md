I wanted to make a standalone copy of the rainbow tutorial: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/adafruit-kb2040/examples/adafruit_kb2040_rainbow.rs

Made sure to keep all dependencies the same in Cargo.toml.
Copied the config.toml and the memory.x file. Added vscode settings.json so rust-analyzer would work.

plug in the KB2040 and get it into bootloader mode by pressing the reset button
while you hold down the boot button.
`cargo run` will automatically use elf2uf2-rs to convert .elf to .uf2 and upload to the board.

[The Rusty Bits: Embedded Rust Setup Explained](https://www.youtube.com/watch?v=TOAynddiu5M)

[Rp2040_HAL I2C](https://docs.rs/rp2040-hal/0.10.0/rp2040_hal/i2c/index.html)