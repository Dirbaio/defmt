//! `defmt` global logger over semihosting
//!
//! NOTE this is meant to only be used with QEMU
//!
//! WARNING using `cortex_m_semihosting`'s `hprintln!` macro or `HStdout` API will corrupt `defmt`
//! log frames so don't use those APIs.

#![no_std]

use core::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use cortex_m::{interrupt, register};
use cortex_m_semihosting::hio;

#[defmt::global_logger]
struct Logger;

static TAKEN: AtomicUsize = AtomicUsize::new(0);
static INTERRUPTS_ACTIVE: AtomicBool = AtomicBool::new(false);

unsafe impl defmt::Logger for Logger {
    fn acquire() {
        let primask = register::primask::read();
        interrupt::disable();
        let taken = TAKEN.load(Ordering::Relaxed);
        TAKEN.store(taken + 1, Ordering::Relaxed);
        if taken == 0 {
            INTERRUPTS_ACTIVE.store(primask.is_active(), Ordering::Relaxed);
        }
    }

    unsafe fn release() {
        let taken = TAKEN.load(Ordering::Relaxed);
        TAKEN.store(taken - 1, Ordering::Relaxed);
        if taken == 1 {
            if INTERRUPTS_ACTIVE.load(Ordering::Relaxed) {
                // re-enable interrupts
                interrupt::enable()
            }
        }
    }

    unsafe fn write(bytes: &[u8]) {
        if TAKEN.load(Ordering::Relaxed) == 1 {
            // using QEMU; it shouldn't mind us opening several handles (I hope)
            if let Ok(mut hstdout) = hio::hstdout() {
                hstdout.write_all(bytes).ok();
            }
        }
    }
}
