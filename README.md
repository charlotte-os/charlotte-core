# CharlotteCore - The Kernel of the Charlotte Project

CharlotteCore is an OS kernel designed to present system resources to application software through an interface that is as low level as possible while representing the least common denominator in terms of functionality for each device type. Each interface may also offer queryable extensions that depend on the specific underlying hardware. Operations on these virtual device interfaces are translated into equivalent operations on the underlying hardware through the kernel and its drivers. We refer to this design as the transitive kernel architecture.

Much like with with the exokernel architecture the transitive kernel architecture allows for significantly greater performance optimization and flexibility in userspace software design than is possible on traditional operating systems however unlike exokernels it reaps the benefits of a single set of well maintained drivers for actual hardware and is also more simple, stable, and secure.

### Status

CharlotteCore is in very early development. Run it at your own risk.

### Implementation Languages

CharlotteCore is written in Rust and Assembly language

### Target Platforms

- x86-64 PCs with UEFI and ACPI (In Progress)
- Aarch64 devices with UEFI and ACPI (Planned)

### Building and Testing

#### Building

```bash
make build-debug    # debug
make build-release  # release
```
#### Testing

```bash
make test-debug    # For testing a debug build in qemu
make test-release  # For testing a release build in qemu
```
### Documentation

Detailed documentation will eventually be available on the project's wiki.

### Contributing

Contributions are welcome and highly appreciated. Please create a new branch for your work and submit a merge request on GitHub. Make sure to rebase all your changes on master to maintain a linear commit history, and avoid merge commits.

### Licensing
This project is licensed under the GNU General Public License Version 3.0 or later as published by the Free Software Foundation. See the LICENSE file for details.