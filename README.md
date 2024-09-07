# `cbof`

## The Kernel of CharlotteOS

![Testing](https://github.com/charlotte-os/charlotte-core/actions/workflows/test_code.yml/badge.svg)
![Vulnerability](https://github.com/charlotte-os/charlotte-core/actions/workflows/cron_report.yml/badge.svg)

`cbof` is the kernel of CharlotteOS. It is a monolithic kernel that is designed to provide a flexible and low level interface to program running ontop of it.

### Status

`cbof` is the successor to Charlotte Core our previous kernel in development. The kernel was redeveloped due to a
lack of sufficient modularity, compatibility with the C ABI in all public interfaces, and improper code organization
which could not be easily refactored in place. `cbof` is the result of that redevelopment though it does reuse a
substantial amount of code from Charlotte Core. A fork of Charlotte Core is available as an archive in the CharlotteOS GitHub organization.

`cbof` is in very early development and does not yet support a userspace. Run it at your own risk.

### Implementation Languages

`cbof` is written in Rust and Assembly language

### Target Platforms

- x86_64-unknown-none
- aarch64-unknown-none
  - Firmware must support either UEFI and ACPI or EBBR

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

The `x86_64` portion of any of the previous commands can be replaced with `aarch64` or `riscv64` to build the kernel for the Arm and RISC-V architectures respectively however it should be noted that support for these ISAs is much less complete than for x86_64 for the time being.

#### GDB debug probe

Follow the steps in the `Building` section above, replacing `make run-x86_64-debug` for `run-x86_64-debugprobe`, this will start qemu, but it will appear unresponsive
this is because it's waiting for gdb to connect, on linux this can be achieved by in another terminal running:

```bash
gdb charlotte_core/target/x86_64-unknown-none/debug/charlotte_core
```

and once gdb is open:

```gdb
(gdb) target remote localhost:1234
make sure to set some breakpoints or the code will just go straight to the halt at the end of main currently
(gdb) c
```

*OR*
Use the .gdbinit file present in the repo, to do this you need to allow gdb to load the .gdbinit file,
this can be accomplished by adding `add-auto-load-safe-path [path to the repo here]/.gdbinit` to `$HOME/.config/gdb/gdbinit`, wit this done you just need to run:

```bash
# in terminal 1
make run-x86_64-debugprobe
# in another terminal
gdb
```

if you are currently in the repo main folder you may use the snippet bellow to add the loading allow to gdbinit

```bash
mkdir -p $HOME/.config/gdb/;echo "add-auto-load-safe-path $(pwd)/.gdbinit" >> $HOME/.config/gdb/gdbinit
```

further reference [Qemu GDB docs](https://qemu-project.gitlab.io/qemu/system/gdb.html)

### Documentation

Detailed documentation will eventually be available in this repository's GitHub wiki and at [CharlotteOS's website](https://www.charlotte-os.org/).

### Contributing

Contributions are always welcome and highly appreciated. Please create a new branch for your work and submit a pull request on GitHub. Make sure to rebase all your changes on master to maintain a linear commit history avoiding merge commits to the extent possible. Feel free to grab any unassigned issues in the GitHub issue tracker for this repository.

Also please join [our community on Discord](https://discord.com/invite/vE7bCCKx4X) to stay up to date with the latest on CharlotteOS development.
