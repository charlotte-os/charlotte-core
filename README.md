# Charlotte Core
### The Kernel of CharlotteOS

Charlotte Core is a hybrid operating system kernel developed part of of CharlotteOS.

### Status

Charlotte Core is in very early development. Run it at your own risk.

### Implementation Languages

Charlotte Core is written in Rust and Assembly language

### Target Platforms

- x86-64 PCs with UEFI and ACPI (In Progress)
- Aarch64 devices with UEFI and ACPI (In Progress)
- RISC-V devices with UEFI and ACPI (Under Consideration)

### Building and Testing

#### Building

```bash
make build-x86_64-debug    # debug
make build-x86_64-release  # release
```
#### Testing

```bash
make run-x86_64-debug    # For testing a debug build in qemu
make run-x86_64-release  # For testing a release build in qemu
```
### Documentation

Detailed documentation will eventually be available on the project's wiki.

### Contributing

Contributions are welcome and highly appreciated. Please create a new branch for your work and submit a pull request on GitHub. Make sure to rebase all your changes on master to maintain a linear commit history avoiding merge commits to the extent possible.
