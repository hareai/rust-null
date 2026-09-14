# rust-null

A userspace model of Linux `/dev/null`, on the way to a Rust-for-Linux `/dev/rust-null` device.

[简体中文](README.zh-CN.md)

## Quick start

```bash
git clone https://github.com/hareai/rust-null.git
cd rust-null
cargo test
echo hello | cargo run --example discard
```

Rust 1.80 or newer. `cargo test` is the userspace contract; it does not load a kernel module.

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

## Roadmap

1. **Userspace** (this tree) — `NullDevice` + contract tests.
2. **Rust for Linux** — out-of-tree module against a `CONFIG_RUST=y` kernel.
3. **`/dev/rust-null`** — misc device with the same read/write/llseek contract.

## License

[GPL-2.0-only](LICENSE)
