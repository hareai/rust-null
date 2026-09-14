//! Cross-check the userspace model against this machine's `/dev/null`.
//!
//! The kernel device is `1:3`. These tests are the live oracle for the
//! userspace crate. They fail if `/dev/null` cannot be opened.

#![cfg(unix)]

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};

use rust_null::NullDevice;

fn open_linux_null() -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open("/dev/null")
}

#[test]
fn linux_null_read_is_eof_and_does_not_fill() {
    let mut linux = open_linux_null().expect("/dev/null");
    let mut buf = [0xAAu8; 16];
    assert_eq!(linux.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [0xAA; 16]);
}

#[test]
fn linux_null_write_returns_count() {
    let mut linux = open_linux_null().expect("/dev/null");
    assert_eq!(linux.write(b"hello").unwrap(), 5);
    assert_eq!(linux.write(&[0u8; 65_536]).unwrap(), 65_536);
}

#[test]
fn linux_null_seek_always_returns_zero() {
    let mut linux = open_linux_null().expect("/dev/null");
    assert_eq!(linux.seek(SeekFrom::Start(100)).unwrap(), 0);
    assert_eq!(linux.seek(SeekFrom::Current(5)).unwrap(), 0);
    assert_eq!(linux.seek(SeekFrom::Current(-10)).unwrap(), 0);
    assert_eq!(linux.seek(SeekFrom::End(7)).unwrap(), 0);
    assert_eq!(linux.seek(SeekFrom::End(-1)).unwrap(), 0);
}

#[test]
fn model_matches_linux_null() {
    let mut model = NullDevice::new();
    let mut linux = open_linux_null().expect("/dev/null");

    let payload = b"abcdef";
    assert_eq!(model.write(payload).unwrap(), linux.write(payload).unwrap());

    let mut a = [0xAAu8; 8];
    let mut b = [0xAAu8; 8];
    assert_eq!(model.read(&mut a).unwrap(), linux.read(&mut b).unwrap());
    assert_eq!(a, b);

    for pos in [
        SeekFrom::Start(100),
        SeekFrom::Current(5),
        SeekFrom::Current(-10),
        SeekFrom::End(7),
        SeekFrom::End(-1),
    ] {
        assert_eq!(
            model.seek(pos).unwrap(),
            linux.seek(pos).unwrap(),
            "{pos:?}"
        );
    }
}
