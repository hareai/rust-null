// SPDX-License-Identifier: GPL-2.0

//! Rust-for-Linux `MiscDevice` with the `/dev/null` contract.
//!
//! This file is the out-of-tree module you would build against a
//! `CONFIG_RUST=y` kernel (`make LLVM=1` in that tree, `M=` this
//! directory, `obj-m := rust_null.o` compiling *this* `.rs`).
//!
//! Distro kernels often ship `CONFIG_HAVE_RUST=y` but not `CONFIG_RUST`.
//! They cannot compile this file. The loadable module in that case is
//! [`rust_null.c`](../rust_null.c), which registers the same
//! `struct miscdevice` name (`rust-null`) and the same
//! `file_operations` (`read` / `write` / `read_iter` / `write_iter` /
//! `llseek`).
//!
//! `kernel::miscdevice::MiscDevice` (mainline) currently hooks
//! `read_iter` / `write_iter` / `ioctl` / `mmap`. It does **not**
//! expose `.llseek`. `/dev/null` uses `null_lseek` (`f_pos = 0`).
//! The C module implements that; a future RfL `llseek` hook should
//! match it. Until then, a pure `MiscDevice` would not pass the seek
//! half of `tests/compare_devices.rs`.

use kernel::{
    fs::{File, Kiocb},
    iov::{IovIterDest, IovIterSource},
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    prelude::*,
};

module! {
    type: RustNullModule,
    name: "rust_null",
    authors: ["evie"],
    description: "misc device with the /dev/null I/O contract",
    license: "GPL",
}

#[pin_data]
struct RustNullModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<RustNull>,
}

impl kernel::InPlaceModule for RustNullModule {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("rust_null: registering misc device rust-null\n");

        let options = MiscDeviceOptions { name: c"rust-null" };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options),
        })
    }
}

/// Per-open state. `/dev/null` has none; neither do we.
struct RustNull;

#[vtable]
impl MiscDevice for RustNull {
    type Ptr = ();

    fn open(_file: &File, _misc: &MiscDeviceRegistration<Self>) -> Result<Self::Ptr> {
        Ok(())
    }

    /// `read_null` / `read_iter_null`: always EOF. Do not fill `iov`.
    fn read_iter(_kiocb: Kiocb<'_, Self::Ptr>, _iov: &mut IovIterDest<'_>) -> Result<usize> {
        Ok(0)
    }

    /// `write_null` / `write_iter_null`: accept every byte, discard it.
    fn write_iter(_kiocb: Kiocb<'_, Self::Ptr>, iov: &mut IovIterSource<'_>) -> Result<usize> {
        let count = iov.len();
        iov.advance(count);
        Ok(count)
    }
}
