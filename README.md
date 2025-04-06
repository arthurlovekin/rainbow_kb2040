# Install 
```
rustup target add thumbv6m-none-eabi # install instruction-set for the RP2040
cargo install elf2uf2-rs # globally install tool to automatically flash the board
cargo build
```
To Flash and Run: Get the KB2040 into bootloader mode by pressing the RST button while holding down the BOOT button. Then `cargo run` (this automatically uses elf2uf2-rs to convert .elf to .uf2 and upload to the board).

# Notes
I wanted to make a standalone copy of the rainbow tutorial: https://github.com/rp-rs/rp-hal-boards/blob/main/boards/adafruit-kb2040/examples/adafruit_kb2040_rainbow.rs

Made sure to keep all dependencies the same in Cargo.toml.
Copied the config.toml and the memory.x file. Added vscode settings.json so rust-analyzer would work.

[The Rusty Bits: Embedded Rust Setup Explained](https://www.youtube.com/watch?v=TOAynddiu5M)