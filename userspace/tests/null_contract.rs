//! Contract tests for the userspace `/dev/null` model.
//!
//! These assertions follow Linux `drivers/char/mem.c`:
//! - `read_null` returns 0
//! - `write_null` returns `count` and ignores the buffer
//! - `null_lseek` does `return file->f_pos = 0`

use std::io::{Read, Seek, SeekFrom, Write};

use rust_null::NullDevice;

#[test]
fn read_is_always_eof_and_does_not_touch_the_buffer() {
    let mut n = NullDevice::new();
    let mut buf = [0xAAu8; 16];
    assert_eq!(n.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [0xAA; 16]);
}

#[test]
fn write_discards_bytes_and_reports_the_full_count() {
    let mut n = NullDevice::new();
    assert_eq!(n.write(b"").unwrap(), 0);
    assert_eq!(n.write(b"hello").unwrap(), 5);
    assert_eq!(n.write(&[0u8; 65_536]).unwrap(), 65_536);
}

#[test]
fn written_bytes_cannot_be_read_back() {
    let mut n = NullDevice::new();
    n.write_all(b"secret").unwrap();
    let mut buf = [1u8; 4];
    assert_eq!(n.read(&mut buf).unwrap(), 0);
    assert_eq!(buf, [1, 1, 1, 1]);
}

#[test]
fn seek_ignores_offset_and_whence_and_returns_zero() {
    let mut n = NullDevice::new();
    assert_eq!(n.seek(SeekFrom::Start(100)).unwrap(), 0);
    assert_eq!(n.seek(SeekFrom::Current(5)).unwrap(), 0);
    assert_eq!(n.seek(SeekFrom::Current(-10)).unwrap(), 0);
    assert_eq!(n.seek(SeekFrom::End(7)).unwrap(), 0);
    assert_eq!(n.seek(SeekFrom::End(-1)).unwrap(), 0);
    assert_eq!(n.stream_position().unwrap(), 0);
}

#[test]
fn write_does_not_move_the_file_position() {
    let mut n = NullDevice::new();
    n.write_all(b"abc").unwrap();
    assert_eq!(n.stream_position().unwrap(), 0);
}

#[test]
fn kernel_named_helpers_match_mem_c() {
    let n = NullDevice::new();
    let mut buf = [0xFFu8; 8];
    assert_eq!(n.read_null(&mut buf), 0);
    assert_eq!(buf, [0xFF; 8]);
    assert_eq!(n.write_null(b"xyz"), 3);
    assert_eq!(n.null_lseek(100, 0), 0);
    assert_eq!(n.null_lseek(-10, 1), 0);
    assert_eq!(n.null_lseek(7, 2), 0);
}
