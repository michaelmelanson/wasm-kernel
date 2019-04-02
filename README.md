# WASM kernel

**tl;dr:** Follow the _Setup instructions_ then run `make`. 

It'll run a simple UEFI binary (see `extern/init`) in QEMU that executes WebAssembly.

![Screenshot](doc/images/screenshot.png)

## Setup instructions

```
brew install qemu
cargo install cargo-xbuild
rustup toolchain add nightly-2019-03-16
rustup component add wasm32-unknown-unknown
```
