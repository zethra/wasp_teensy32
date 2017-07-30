# Wasp
## A 3D printer firmware written in Rust

## Waring: WIP
Wasp is not done. Not even close.

### Goals
 - Simple: Parse and execute Gcodes. Nothing else
 - Host friendly: Be clear about what is happening and what the printer is doing
 - Safety: Never crash, always be in control of the printer

### Supported boards
 - Teensy 3.2

### How to build

Install:
 - Current nightly build of Rust
 - Install the arm-unknown-linux-gnueabihf toolchain for rust (using rustup)
 - libnewlib-arm-none-eabi
 - [Xargo](https://github.com/japaric/xargo)
 - The arm-none-eabi-gcc toolchain
 - Clang, see [rust-bindgen’s requirements](https://github.com/servo/rust-bindgen#requirements).
 - [teensy-loader-cli](https://www.pjrc.com/teensy/loader_cli.html) for flashing your program onto hardware.

### Rust Language Server
The [Rust Language Server](https://github.com/rust-lang-nursery/rls) will analyse the code and provide auto completions and information to code editors. In order to work correctly with xargo, an rls.toml file is included that should make it work. You will also need to build the code at least once before RLS will work.