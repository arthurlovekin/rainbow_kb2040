# KB2040 Rainbow Tutorial
Standalone copy of the rainbow tutorial: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/adafruit-kb2040/examples/adafruit_kb2040_rainbow.rs

## Setup
Install the RP2040 toolchain: 
```bash
rustup target add thumbv6m-none-eabi 
cargo install elf2uf2-rs
```

Plug in the KB2040 and get it into bootloader mode by pressing the reset button while you hold down the boot button.
Run `cargo run` to flash the board. This uses elf2uf2-rs to convert .elf to .uf2 and upload to the board.

## Dev Notes
Made sure to keep all dependencies the same in Cargo.toml.
Copied the config.toml and the memory.x file. Added vscode settings.json so rust-analyzer would work.

## References
[The Rusty Bits: Embedded Rust Setup Explained](https://www.youtube.com/watch?v=TOAynddiu5M)