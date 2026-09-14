# rust-null

A userspace model of Linux `/dev/null`, plus a misc device `/dev/rust-null` with the same I/O contract.

[简体中文](README.zh-CN.md)

## Quick start

```bash
git clone https://github.com/hareai/rust-null.git
cd rust-null
cargo test
echo hello | cargo run -p rust-null --example discard
```

Rust 1.80 or newer. `cargo test` is the userspace contract. It does not load a kernel module. `/dev/null` vs `/dev/rust-null` tests skip unless the device node exists.

## Usage

`NullDevice` is the same three operations as `drivers/char/mem.c`:

```rust
use std::io::{Read, Write};
use rust_null::NullDevice;

let mut n = NullDevice::new();
assert_eq!(n.write(b"secret").unwrap(), 6); // discarded
let mut buf = [0xAAu8; 4];
assert_eq!(n.read(&mut buf).unwrap(), 0);   // EOF, buffer untouched
```

| Call | Result |
|---|---|
| `read` | `0` (EOF). The buffer is not filled. |
| `write` | `count`. Bytes are thrown away. |
| `seek` | `0`. Offset and whence are ignored. |

That last row is the surprising one: `lseek` on `/dev/null` always returns `0`, even for `SEEK_END`.

## Tree

```text
userspace/   NullDevice crate and contract tests
kernel/      loadable misc device (C) and RfL source (not built here)
```

The C module registers a misc device named `rust-null` (`misc_register`, major 10, dynamic minor) whose `file_operations` copy `null_fops`. That is the same VFS path Rust-for-Linux `MiscDevice` wraps.

```bash
make -C kernel            # needs linux-headers for the running kernel
# make -C kernel load     # insmod; only on a machine you intend to test
RUST_NULL_REQUIRE_DEV=1 cargo test -p rust-null --test compare_devices --locked
```

A distro kernel without `CONFIG_RUST=y` cannot compile `kernel/rfl/rust_null.rs`. That file is the out-of-tree module for a self-built Rust-enabled kernel (`make LLVM=1`).

## License

[GPL-2.0-only](LICENSE)
