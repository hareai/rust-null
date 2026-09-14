//! Userspace model of Linux `/dev/null`.
//!
//! The behavior is copied from `drivers/char/mem.c`, not invented:
//!
//! | Kernel helper   | Contract                                      |
//! |-----------------|-----------------------------------------------|
//! | `read_null`     | return `0` (EOF). Do not fill the buffer.     |
//! | `write_null`    | return `count`. Discard the bytes.            |
//! | `null_lseek`    | `file->f_pos = 0`; return `0`. Ignore whence. |
//!
//! This crate is the userspace model (`userspace/`). The matching misc
//! device is `kernel/rust_null.c` (`/dev/rust-null`). The I/O contract
//! stays the same; only the transport changes (`std::io` here,
//! `file_operations` / RfL `MiscDevice` there).

use std::io::{self, Read, Seek, SeekFrom, Write};

/// Stand-in for the kernel's `/dev/null` character device.
///
/// The type holds no buffer. A write cannot be read back. Seeking
/// never produces a non-zero offset, matching `null_lseek`.
///
/// ```
/// use std::io::{Read, Write};
/// use rust_null::NullDevice;
///
/// let mut n = NullDevice::new();
/// assert_eq!(n.write(b"secret").unwrap(), 6);
/// let mut buf = [0xAAu8; 4];
/// assert_eq!(n.read(&mut buf).unwrap(), 0);
/// assert_eq!(buf, [0xAA; 4]);
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NullDevice;

impl NullDevice {
    /// Open a new `/dev/null` handle. There is no per-open state.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// `read_null`: always EOF.
    ///
    /// The kernel implementation is `return 0;` and does not write
    /// through the user pointer. The buffer is left unchanged.
    #[must_use]
    pub fn read_null(&self, _buf: &mut [u8]) -> usize {
        0
    }

    /// `write_null`: accept every byte and throw it away.
    ///
    /// The kernel implementation is `return count;`.
    #[must_use]
    pub fn write_null(&self, buf: &[u8]) -> usize {
        buf.len()
    }

    /// `null_lseek`: ignore `offset` / `orig`, report position 0.
    ///
    /// Kernel:
    /// ```c
    /// return file->f_pos = 0;
    /// ```
    ///
    /// `orig` is the kernel's `SEEK_*` value (`0`/`1`/`2`). It is
    /// accepted and ignored, including `SEEK_END` with a negative
    /// offset, which would fail on a regular file.
    #[must_use]
    pub fn null_lseek(&self, _offset: i64, _orig: i32) -> u64 {
        0
    }
}

impl Read for NullDevice {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(self.read_null(buf))
    }
}

impl Write for NullDevice {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(self.write_null(buf))
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Seek for NullDevice {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let (offset, orig) = match pos {
            SeekFrom::Start(n) => (i64::try_from(n).unwrap_or(i64::MAX), 0),
            SeekFrom::Current(n) => (n, 1),
            SeekFrom::End(n) => (n, 2),
        };
        Ok(self.null_lseek(offset, orig))
    }
}
