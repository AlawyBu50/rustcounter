// SPDX-License-Identifier: GPL-2.0
//! rustcounter: a Rust character device counter via /dev/rustcounter.
//!
//! Each write increments a kernel-space counter.
//! Each read returns the current counter value.

use kernel::{
    fs::{File, Kiocb},
    iov::{IovIterDest, IovIterSource},
    miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration},
    prelude::*,
    str::CString,
};

module! {
    type: RustCounter,
    name: "rustcounter",
    authors: ["Ali Bukhamseen"],
    description: "rustcounter  a counter character device",
    license: "GPL",
}

struct CounterState {
    count: u64,
    consumed: bool,
}

kernel::sync::global_lock! {
    // SAFETY: Initialized in module initializer before first use.
    unsafe(uninit) static STATE: Mutex<CounterState> = CounterState {
        count: 0,
        consumed: false,
    };
}

#[pin_data]
struct RustCounter {
    #[pin]
    _miscdev: MiscDeviceRegistration<RustCounterDevice>,
}

impl kernel::InPlaceModule for RustCounter {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("rustcounter: module loaded\n");

        // SAFETY: Called exactly once during module init.
        unsafe { STATE.init() };

        let opts = MiscDeviceOptions { name: c"rustcounter" };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(opts),
        })
    }
}

struct RustCounterDevice;

#[vtable]
impl MiscDevice for RustCounterDevice {
    type Ptr = Pin<KBox<Self>>;

    fn open(_file: &File, _misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        Ok(KBox::new(RustCounterDevice, GFP_KERNEL).map(KBox::into_pin)?)
    }

    fn write_iter(
        mut kiocb: Kiocb<'_, Self::Ptr>,
        iov: &mut IovIterSource<'_>,
    ) -> Result<usize> {
        let mut sink = KVec::new();

        let len = iov.copy_from_iter_vec(&mut sink, GFP_KERNEL)?;

        *kiocb.ki_pos_mut() = 0;

        let mut state = STATE.lock();
        state.count += 1;
        state.consumed = false;

        pr_info!("rustcounter: incremented to {}\n", state.count);

        Ok(len)
    }

    fn read_iter(
        mut kiocb: Kiocb<'_, Self::Ptr>,
        iov: &mut IovIterDest<'_>,
    ) -> Result<usize> {
        let mut state = STATE.lock();

        if state.consumed {
            return Ok(0);
        }

        let formatted = CString::try_from_fmt(fmt!("{}\n", state.count))?;
        let bytes = formatted.to_bytes();

        let n = iov.simple_read_from_buffer(kiocb.ki_pos_mut(), bytes)?;

        state.consumed = true;

        pr_info!("rustcounter: read count {}\n", state.count);

        Ok(n)
    }
}
