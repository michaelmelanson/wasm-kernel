# ossify

**tl;dr:** Follow the _Setup instructions_ then run `make`. It'll run a simple UEFI binary (see `extern/init`) in QEMU that executes WebAssembly.

## Setup instructions

```
brew install qemu
cargo install cargo-xbuild
rustup toolchain add nightly
rustup component add wasm32-unknown-unknown
```