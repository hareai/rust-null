# `/dev/null` contract

Userspace `NullDevice` copies three helpers from Linux `drivers/char/mem.c`. It is not a file, not a buffer, and not `/dev/zero`.

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

## Userspace mapping

| Kernel | `NullDevice` | `std::io` |
|---|---|---|
| `read_null` | `read_null` | `Read::read` → `Ok(0)` |
| `write_null` | `write_null` | `Write::write` → `Ok(count)` |
| `null_lseek` | `null_lseek` | `Seek::seek` → `Ok(0)` |

## Not this device

- `/dev/zero` fills reads with `0x00`.
- `/dev/full` fails writes with `ENOSPC`.
- A regular file of length 0 still has a moving offset after `SEEK_SET`.

Live Linux `/dev/null` (`1:3`) matches the table above: `lseek(SEEK_SET, 100)` returns `0`.

## Next

The out-of-tree Rust-for-Linux module will register a misc device named `rust-null` and implement the same three operations on `kernel::file` types. The contract tests here stay the oracle.
