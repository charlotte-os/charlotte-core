# Charlotte Core
### The Kernel of CharlotteOS

Charlotte Core is a hybrid operating system kernel developed as a part of CharlotteOS.

### Status

Charlotte Core is in very early development. Run it at your own risk.

### Implementation Languages

Charlotte Core is written in Rust and Assembly language

### Target Platforms

- x86-64 PCs with UEFI and ACPI (In Progress)
- Aarch64 devices with UEFI and ACPI (In Progress)
- RISC-V devices with UEFI and ACPI (Not under active development yet)

#### Building
The following commands are used to build the debug and release versions of the kernel.
```bash
make build-x86_64-debug    # debug
make build-x86_64-release  # release
```
#### Testing

```bash
make run-x86_64-debug    # For testing a debug build in qemu
make run-x86_64-release  # For testing a release build in qemu
```

The `x86_64` portion of any of the previous commands can be replaced with `aarch64` or `riscv64` to build the kernel for the Arm and RISC-V architectures respectively however it should be noted that support for these ISAs is much less complete
than for x86_64 for the time being.

### Documentation

Detailed documentation will eventually be available on the project's wiki and website.

### Contributing

Contributions are welcome and highly appreciated. Please create a new branch for your work and submit a pull request on GitHub. Make sure to rebase all your changes on master to maintain a linear commit history avoiding merge commits to the extent possible.
