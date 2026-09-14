# rust-null

Linux `/dev/null` 的用户态模型，以及合同相同的 misc 设备 `/dev/rust-null`。

[English](README.md)

## 快速开始

```bash
git clone https://github.com/hareai/rust-null.git
cd rust-null
cargo test
echo hello | cargo run -p rust-null --example discard
```

需要 Rust 1.80 或更新。`cargo test` 只验证用户态合同，不会加载内核模块。没有 `/dev/rust-null` 时，设备对比测试会跳过。

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

## 目录

```text
userspace/   NullDevice crate 和合同测试
kernel/      可加载的 misc 设备（C）和 RfL 源码（这里不编）
```

C 模块用 `misc_register` 注册名为 `rust-null` 的 misc 设备（主设备号 10，动态次设备号），`file_operations` 抄 `null_fops`。这就是 Rust-for-Linux `MiscDevice` 封装的那条 VFS 路径。

```bash
make -C kernel            # 需要正在运行的内核的 linux-headers
# make -C kernel load     # insmod；只在打算测试的机器上
RUST_NULL_REQUIRE_DEV=1 cargo test -p rust-null --test compare_devices --locked
```

发行核没有 `CONFIG_RUST=y` 时编不了 `kernel/rfl/rust_null.rs`。那是给自己编的 Rust 内核用的 out-of-tree 模块（`make LLVM=1`）。

## 许可证

[GPL-2.0-only](LICENSE)
