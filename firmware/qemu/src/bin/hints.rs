#![no_std]
#![no_main]

use core::sync::atomic::{AtomicU32, Ordering};

use cortex_m_rt::entry;
use cortex_m_semihosting::debug;

use defmt_semihosting as _; // global logger

#[entry]
fn main() -> ! {
    let x = 42;
    defmt::info!("no hint {=u8}", x);
    defmt::info!("hex     {=u8:x}", x);
    defmt::info!("HEX     {=u8:X}", x);
    defmt::info!("binary  {=u8:b}", x);
    defmt::info!("ASCII   {=u8:a}", x);
    defmt::info!("Debug   {=u8:?}", x);

    defmt::info!("----");

    let x = 42;
    defmt::info!("no-hint {=i8}", x);
    defmt::info!("hex     {=i8:x}", x);
    defmt::info!("HEX     {=i8:X}", x);
    defmt::info!("binary  {=i8:b}", x);
    defmt::info!("ASCII   {=i8:a}", x);
    defmt::info!("Debug   {=i8:?}", x);

    loop {
        debug::exit(debug::EXIT_SUCCESS)
    }
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    // monotonic counter
    static COUNT: AtomicU32 = AtomicU32::new(0);
    COUNT.fetch_add(1, Ordering::Relaxed) as u64
}

// like `panic-semihosting` but doesn't print to stdout (that would corrupt the defmt stream)
#[cfg(target_os = "none")]
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE)
    }
}
