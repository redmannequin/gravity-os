# gravity-os
A pet kernel to explore and create a single purpose os.

build
---
Before that ensure tou have the folloing installed:
- `rust` 
- `aarch64-unknown-none-softfloat` target for rust
- `make`
- `aarch64-none-elf`

To build the kernel run `make` at the root of the project. This will create
the kernel image(`kernel8.img`)

qemu
---
Note: to run QEMU ensure the `MMIO_BASE` is set to `MMIO_BASE_RPI_3`.

To start `qemu` run the following command `qemu-system-aarch64 -M raspi3b -kernel kernel/kernel8.img -serial stdio`

resources
---
https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads