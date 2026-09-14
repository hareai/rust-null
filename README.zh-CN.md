# rust-null

Linux `/dev/null` 的用户态模型，目标是做成 Rust-for-Linux 的 `/dev/rust-null`。

[English](README.md)

## 快速开始

```bash
git clone https://github.com/hareai/rust-null.git
cd rust-null
cargo test
echo hello | cargo run --example discard
```

需要 Rust 1.80 或更新。`cargo test` 只验证用户态合同，不会加载内核模块。

## 用法

`NullDevice` 对应 `drivers/char/mem.c` 里的三个操作：

```rust
use std::io::{Read, Write};
use rust_null::NullDevice;

let mut n = NullDevice::new();
assert_eq!(n.write(b"secret").unwrap(), 6); // 丢掉
let mut buf = [0xAAu8; 4];
assert_eq!(n.read(&mut buf).unwrap(), 0);   // EOF，缓冲区不动
```

| 调用 | 结果 |
|---|---|
| `read` | `0`（EOF）。不填充缓冲区。 |
| `write` | `count`。字节被丢掉。 |
| `seek` | `0`。offset 和 whence 都被忽略。 |

最后一行容易踩：`/dev/null` 上的 `lseek` 永远返回 `0`，包括 `SEEK_END`。

## 路线

1. **用户态**（当前树）— `NullDevice` + 合同测试。
2. **Rust for Linux** — 对着 `CONFIG_RUST=y` 的内核做 out-of-tree 模块。
3. **`/dev/rust-null`** — 同样 read/write/llseek 合同的 misc 设备。

## 许可证

[GPL-2.0-only](LICENSE)
