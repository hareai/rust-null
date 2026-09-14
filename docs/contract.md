# `/dev/null` contract

Userspace `NullDevice` and the `rust-null` misc device copy three helpers from Linux `drivers/char/mem.c`. They are not a file, not a buffer, and not `/dev/zero`.

## Kernel

```c
static ssize_t read_null(struct file *file, char __user *buf,
                         size_t count, loff_t *ppos)
{
        return 0;
}

static ssize_t write_null(struct file *file, const char __user *buf,
                          size_t count, loff_t *ppos)
{
        return count;
}

static loff_t null_lseek(struct file *file, loff_t offset, int orig)
{
        return file->f_pos = 0;
}
```

`ppos` / `offset` / `orig` are unused. A successful write does not move the file position, because the next seek (and the implicit position) is always zero.

`null_fops` also sets `read_iter` / `write_iter` to the same idea (`return 0` / `return count` after `iov_iter_advance`).

## Userspace mapping

| Kernel | `NullDevice` | `std::io` |
|---|---|---|
| `read_null` | `read_null` | `Read::read` → `Ok(0)` |
| `write_null` | `write_null` | `Write::write` → `Ok(count)` |
| `null_lseek` | `null_lseek` | `Seek::seek` → `Ok(0)` |

## `/dev/null` vs `/dev/rust-null`

| | `/dev/null` | `/dev/rust-null` |
|---|---|---|
| Class | mem char `1:3` | misc, major 10, dynamic minor |
| Register | `chr_dev_init` / `devlist[3]` | `misc_register` (`name = "rust-null"`) |
| Contract | `null_fops` | same read / write / llseek / `*_iter` |

`userspace/tests/compare_devices.rs` opens both and checks the table. Without the device node the tests skip. After `make -C kernel load`:

```bash
RUST_NULL_REQUIRE_DEV=1 cargo test -p rust-null --test compare_devices --locked
```

## Not this device

- `/dev/zero` fills reads with `0x00`.
- `/dev/full` fails writes with `ENOSPC`.
- A regular file of length 0 still has a moving offset after `SEEK_SET`.

Live Linux `/dev/null` (`1:3`) matches the table above: `lseek(SEEK_SET, 100)` returns `0`.
