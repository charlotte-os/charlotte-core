# Charlotte Core

![Testing](https://github.com/charlotte-os/charlotte-core/actions/workflows/test_code.yml/badge.svg)
![Vulnerability](https://github.com/charlotte-os/charlotte-core/actions/workflows/cron_report.yml/badge.svg)

## The Kernel of CharlotteOS

Charlotte Core is the kernel of CharlotteOS, a post Unix operating system.

### Status

Charlotte Core is in early development. Run it at your own risk.

### Implementation Languages

Charlotte Core is written in Rust and Assembly language

### Targets

- x86_64-unknown-none
- aarch64-unknown-none
- riscv64-unknown-none-elf

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

Detailed documentation will eventually be available in this repository's GitHub wiki and at CharlotteOS's website: https://www.charlotte-os.org/

### Contributing

Contributions are always welcome and highly appreciated. Please create a new branch for your work and submit a pull request on GitHub. Make sure to rebase all your changes on master to maintain a linear commit history avoiding merge commits to the extent possible. Feel free to grab any unassigned issues in the GitHub issue tracker for this repository.

Also please join [our community on Discord](https://discord.com/invite/vE7bCCKx4X) to stay up to date with the latest on CharlotteOS development.
