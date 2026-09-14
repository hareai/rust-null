# kernel

Two sources, one device name: `rust-null`.

| File | What it is | When it builds |
|---|---|---|
| `rust_null.c` | `misc_register` + `file_operations` copied from `null_fops` | Distro kernels with `CONFIG_MODULES=y` and matching `linux-headers` |
| `rfl/rust_null.rs` | Rust-for-Linux `MiscDevice` | A self-built kernel with `CONFIG_RUST=y` (`make LLVM=1`) |

`make` in this directory builds **only** the C module (`obj-m := rust_null.o`). It does not compile the `.rs`. Distro headers have no Rust metadata.

## VFS

`/dev/null` is mem `1:3` (`drivers/char/mem.c`). This module is a *misc* device (major 10, `MISC_DYNAMIC_MINOR`). The I/O contract is the same:

```c
read      → 0
write     → count
llseek    → f_pos = 0
read_iter → 0
write_iter → count (iov_iter_advance)
```

`kernel::miscdevice::MiscDevice` on mainline currently wires `read_iter` / `write_iter` / `ioctl` / `mmap`, not `.llseek`. The C module implements `llseek` so a loaded `/dev/rust-null` can match `/dev/null` on seek. A future RfL hook should do the same.

## Commands

```bash
make                # rust_null.ko
make load           # insmod; creates /dev/rust-null
make unload         # rmmod
make clean
```

`load` / `unload` are opt-in. Do not run them on a machine you are not prepared to test.
