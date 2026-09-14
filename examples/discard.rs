//! Read stdin, discard it the same way `/dev/null` would.
//!
//! ```bash
//! echo hello | cargo run --example discard
//! ```

use std::io::{self, Read, Write};

use rust_null::NullDevice;

fn main() -> io::Result<()> {
    let mut null = NullDevice::new();
    let mut stdin = io::stdin().lock();
    let mut buf = [0u8; 8192];
    loop {
        let n = stdin.read(&mut buf)?;
        if n == 0 {
            break;
        }
        null.write_all(&buf[..n])?;
    }
    Ok(())
}
