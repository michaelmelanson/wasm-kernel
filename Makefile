all: extern build qemu

setup:
	brew install qemu

extern:
	cargo build --manifest-path extern/init/Cargo.toml --target "wasm32-unknown-unknown" --release

build:
	cargo +nightly xbuild --target ./x86_64-unknown-uefi.json

dist:
	mkdir -p dist/boot/EFI/boot/
	cp ./target/x86_64-unknown-uefi/debug/ossify.efi dist/boot/EFI/boot/bootx64.efi

qemu: dist
	qemu-system-x86_64 -nodefaults -vga std -monitor vc:1024x768 -machine q35,accel=kvm:tcg -drive if=pflash,format=raw,readonly,file=./ovmf/OVMF.fd  -drive if=pflash,format=raw,file=./ovmf/OVMF_VARS.fd -drive format=raw,file=fat:rw:./dist/boot

.PHONY: all build dist extern qemu
