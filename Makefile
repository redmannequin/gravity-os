default: build

dump: build
	aarch64-none-elf-objdump -D ./target/aarch64-unknown-none/release/gravity-os > dump

build:
	rm -f ./kernel/kernel8.img
	rm -rf target
	cargo rustc --bin gravity-os --release -- -C link-arg=--script=./linker.ld -C target-cpu=cortex-a72
	aarch64-none-elf-objcopy -O binary target/aarch64-unknown-none/release/gravity-os kernel8.img