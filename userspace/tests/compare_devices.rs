//! `/dev/null` vs `/dev/rust-null`.
//!
//! Same I/O contract, different devices: mem char `1:3` versus a misc
//! device named `rust-null`.
//!
//! Without `/dev/rust-null` the tests skip, so `cargo test` stays
//! green in CI. After `make -C kernel load`, run:
//!
//! ```bash
//! RUST_NULL_REQUIRE_DEV=1 cargo test -p rust-null --test compare_devices --locked
//! ```

#![cfg(unix)]

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

const LINUX_NULL: &str = "/dev/null";
const RUST_NULL: &str = "/dev/rust-null";

fn require_dev() -> bool {
    std::env::var_os("RUST_NULL_REQUIRE_DEV").is_some()
}

fn open_rw(path: &str) -> io::Result<File> {
    OpenOptions::new().read(true).write(true).open(path)
}

fn rust_null() -> Option<File> {
    match open_rw(RUST_NULL) {
        Ok(f) => Some(f),
        Err(err) => {
            if require_dev() {
                panic!("{RUST_NULL} is required ({err}); insmod kernel/rust_null.ko");
            }
            eprintln!("skip: {RUST_NULL} ({err})");
            None
        }
    }
}

#[test]
fn rust_null_node_exists() {
    if Path::new(RUST_NULL).exists() {
        return;
    }
    if require_dev() {
        panic!("RED: {RUST_NULL} missing; the misc device is not registered");
    }
    eprintln!("skip: {RUST_NULL} is not registered");
}

#[test]
fn read_is_eof_on_both_and_does_not_fill() {
    let Some(mut rust) = rust_null() else {
        return;
    };
    let mut linux = open_rw(LINUX_NULL).unwrap();
    let mut a = [0xAAu8; 16];
    let mut b = [0xAAu8; 16];
    assert_eq!(linux.read(&mut a).unwrap(), 0);
    assert_eq!(rust.read(&mut b).unwrap(), 0);
    assert_eq!(a, [0xAA; 16]);
    assert_eq!(b, [0xAA; 16]);
}

#[test]
fn write_returns_count_on_both() {
    let Some(mut rust) = rust_null() else {
        return;
    };
    let mut linux = open_rw(LINUX_NULL).unwrap();
    assert_eq!(
        linux.write(b"hello").unwrap(),
        rust.write(b"hello").unwrap()
    );
    assert_eq!(
        linux.write(&[0u8; 65_536]).unwrap(),
        rust.write(&[0u8; 65_536]).unwrap()
    );
}

#[test]
fn written_bytes_cannot_be_read_back_on_either() {
    let Some(mut rust) = rust_null() else {
        return;
    };
    let mut linux = open_rw(LINUX_NULL).unwrap();
    linux.write_all(b"secret").unwrap();
    rust.write_all(b"secret").unwrap();
    let mut a = [1u8; 4];
    let mut b = [1u8; 4];
    assert_eq!(linux.read(&mut a).unwrap(), 0);
    assert_eq!(rust.read(&mut b).unwrap(), 0);
    assert_eq!(a, [1, 1, 1, 1]);
    assert_eq!(b, [1, 1, 1, 1]);
}

#[test]
fn seek_always_returns_zero_on_both() {
    let Some(mut rust) = rust_null() else {
        return;
    };
    let mut linux = open_rw(LINUX_NULL).unwrap();
    for pos in [
        SeekFrom::Start(100),
        SeekFrom::Current(5),
        SeekFrom::Current(-10),
        SeekFrom::End(7),
        SeekFrom::End(-1),
    ] {
        assert_eq!(linux.seek(pos).unwrap(), rust.seek(pos).unwrap(), "{pos:?}");
        assert_eq!(linux.seek(pos).unwrap(), 0, "linux {pos:?}");
        assert_eq!(rust.seek(pos).unwrap(), 0, "rust {pos:?}");
    }
}

#[test]
fn write_does_not_move_position_on_either() {
    let Some(mut rust) = rust_null() else {
        return;
    };
    let mut linux = open_rw(LINUX_NULL).unwrap();
    linux.write_all(b"abc").unwrap();
    rust.write_all(b"abc").unwrap();
    assert_eq!(linux.stream_position().unwrap(), 0);
    assert_eq!(rust.stream_position().unwrap(), 0);
}
