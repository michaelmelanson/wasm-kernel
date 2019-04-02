# WASM kernel

Pushing forward the [_Birth and Death of Javascript_](https://www.destroyallsoftware.com/talks/the-birth-and-death-of-javascript) one step at a time.

**tl;dr:** To run this follow the _Setup instructions_ then run `make`. You'll almost certainly get errors that you'll have to
work through, because this was done in like 4 hours so set your expectations accordingly.

## What is this?

This is a small Rust project that produces a bootable binary with an embedded WebAssembly interpreter. It can boot in QEMU 
using the included UEFI firmware (see `ovmf`), then execute WebAssembly binaries. I haven't tried booting it on real hardware
but theoretically it should work as long as the machine has the right firmware.

It's basically what happens if you took [`uefi-rs`](https://crates.io/crates/uefi) in one hand and 
[`wasmi`](https://crates.io/crates/wasmi) in the other hand, and tried to stick them together. They had an enormous fight, and
when they were done WebAssembly was booting in QEMU.

![Screenshot](doc/images/screenshot.png)

That text comes from a Rust application in `extern/init` that gets compiled to WebAssembly and embedded into the kernel binary then interpreted.

## Setup instructions

```
make setup
brew install qemu
```

At this point, running `make` should start QEMU and show `INFO: Hello from WASM!` as seen in the screenshot above.
