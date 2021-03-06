all: extern build qemu

setup:
	[ -x "`command -v "cargo-xbuild"`" ] || cargo install cargo-xbuild
	rustup target add wasm32-unknown-unknown
	rustup component add rust-src

extern:
	cargo build --manifest-path extern/init/Cargo.toml --target "wasm32-unknown-unknown" --release

build:
	cargo +nightly-2019-03-16 xbuild --target ./x86_64-unknown-uefi.json

dist:
	mkdir -p dist/boot/EFI/boot/
	cp ./target/x86_64-unknown-uefi/debug/wasm-kernel.efi dist/boot/EFI/boot/bootx64.efi

	mkdir -p dist/boot/bin/
	cp ./extern/init/target/wasm32-unknown-unknown/release/init.wasm dist/boot/bin

	zip dist/release.zip dist/boot/ -r

qemu: dist
	qemu-system-x86_64 -nodefaults -vga std -monitor vc:1024x768 -machine q35,accel=kvm:tcg -serial stdio -drive if=pflash,format=raw,readonly,file=./ovmf/OVMF.fd  -drive if=pflash,format=raw,file=./ovmf/OVMF_VARS.fd -drive format=raw,file=fat:rw:./dist/boot

.PHONY: all build dist extern qemu
