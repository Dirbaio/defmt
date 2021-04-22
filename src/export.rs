use crate::{FmtWrite, Format, Formatter, Str};
use core::fmt::Write;

#[cfg(feature = "unstable-test")]
thread_local! {
    static I: core::sync::atomic::AtomicU8 =
        core::sync::atomic::AtomicU8::new(0);
    static T: core::sync::atomic::AtomicU8 =
        core::sync::atomic::AtomicU8::new(0);
}

// NOTE we limit these values to 7-bit to avoid LEB128 encoding while writing the expected answers
// in unit tests
/// For testing purposes
#[cfg(feature = "unstable-test")]
pub fn fetch_string_index() -> u8 {
    I.with(|i| i.load(core::sync::atomic::Ordering::Relaxed)) & 0x7f
}

/// For testing purposes
#[cfg(feature = "unstable-test")]
pub fn fetch_add_string_index() -> usize {
    (I.with(|i| i.fetch_add(1, core::sync::atomic::Ordering::Relaxed)) & 0x7f) as usize
}

#[cfg(feature = "unstable-test")]
pub fn acquire() {
    false
}

#[cfg(not(feature = "unstable-test"))]
#[inline(never)]
pub fn acquire() {
    extern "Rust" {
        fn _defmt_acquire();
    }
    unsafe { _defmt_acquire() }
}

#[cfg(feature = "unstable-test")]
pub fn release() {}

#[cfg(not(feature = "unstable-test"))]
#[inline(never)]
pub fn release() {
    extern "Rust" {
        fn _defmt_release();
    }
    unsafe { _defmt_release() }
}

#[cfg(not(feature = "unstable-test"))]
#[inline(never)]
pub fn write(bytes: &[u8]) {
    extern "Rust" {
        fn _defmt_write(bytes: &[u8]);
    }
    unsafe { _defmt_write(bytes) }
}

/// For testing purposes
#[cfg(feature = "unstable-test")]
pub fn timestamp(_fmt: crate::Formatter<'_>) {}

#[cfg(not(feature = "unstable-test"))]
pub fn timestamp(fmt: crate::Formatter<'_>) {
    extern "Rust" {
        fn _defmt_timestamp(_: crate::Formatter<'_>);
    }
    unsafe { _defmt_timestamp(fmt) }
}

/// Returns the interned string at `address`.
pub fn istr(address: usize) -> Str {
    Str {
        // NOTE address is limited to 14 bits in the linker script
        address: address as *const u8 as u16,
    }
}

mod sealed {
    #[allow(unused_imports)]
    use crate as defmt;
    use crate::{Format, Formatter};
    use defmt_macros::internp;

    pub trait Truncate<U> {
        fn truncate(self) -> U;
    }

    impl Truncate<u8> for u8 {
        fn truncate(self) -> u8 {
            self
        }
    }

    impl Truncate<u8> for u16 {
        fn truncate(self) -> u8 {
            self as u8
        }
    }

    impl Truncate<u8> for u32 {
        fn truncate(self) -> u8 {
            self as u8
        }
    }

    impl Truncate<u8> for u64 {
        fn truncate(self) -> u8 {
            self as u8
        }
    }

    impl Truncate<u8> for u128 {
        fn truncate(self) -> u8 {
            self as u8
        }
    }

    // needed so we can call truncate() without having to check whether truncation is necessary first
    impl Truncate<u16> for u16 {
        fn truncate(self) -> u16 {
            self
        }
    }

    impl Truncate<u16> for u32 {
        fn truncate(self) -> u16 {
            self as u16
        }
    }

    impl Truncate<u16> for u64 {
        fn truncate(self) -> u16 {
            self as u16
        }
    }

    impl Truncate<u16> for u128 {
        fn truncate(self) -> u16 {
            self as u16
        }
    }

    // needed so we can call truncate() without having to check whether truncation is necessary first
    impl Truncate<u32> for u32 {
        fn truncate(self) -> u32 {
            self
        }
    }

    impl Truncate<u32> for u64 {
        fn truncate(self) -> u32 {
            self as u32
        }
    }

    impl Truncate<u32> for u128 {
        fn truncate(self) -> u32 {
            self as u32
        }
    }

    // needed so we can call truncate() without having to check whether truncation is necessary first
    impl Truncate<u64> for u64 {
        fn truncate(self) -> u64 {
            self
        }
    }

    impl Truncate<u64> for u128 {
        fn truncate(self) -> u64 {
            self as u64
        }
    }

    // needed so we can call truncate() without having to check whether truncation is necessary first
    impl Truncate<u128> for u128 {
        fn truncate(self) -> u128 {
            self
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub struct NoneError;

    impl Format for NoneError {
        fn format(&self, _: Formatter) {
            let t = internp!("Unwrap of a None option value");
            defmt::export::write_tag(&t);
        }
    }

    pub trait IntoResult {
        type Ok;
        type Error;
        fn into_result(self) -> Result<Self::Ok, Self::Error>;
    }

    impl<T> IntoResult for Option<T> {
        type Ok = T;
        type Error = NoneError;

        #[inline]
        fn into_result(self) -> Result<T, NoneError> {
            self.ok_or(NoneError)
        }
    }

    impl<T, E> IntoResult for Result<T, E> {
        type Ok = T;
        type Error = E;

        #[inline]
        fn into_result(self) -> Self {
            self
        }
    }
}

pub fn truncate<T>(x: impl sealed::Truncate<T>) -> T {
    x.truncate()
}

pub fn into_result<T: sealed::IntoResult>(x: T) -> Result<T::Ok, T::Error> {
    x.into_result()
}

/// For testing purposes
#[cfg(feature = "unstable-test")]
pub fn panic() -> ! {
    panic!()
}

#[cfg(not(feature = "unstable-test"))]
pub fn panic() -> ! {
    extern "Rust" {
        fn _defmt_panic() -> !;
    }
    unsafe { _defmt_panic() }
}

/// Implementation detail
pub fn write_fmt(f: &impl Format) {
    f.format(Formatter::new());
}

/// Implementation detail
pub fn write_i8(b: &i8) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_i16(b: &i16) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_i32(b: &i32) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_i64(b: &i64) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_i128(b: &i128) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_isize(b: &isize) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_fmt_slice(values: &[impl Format]) {
    write_usize(&values.len());
    for value in values {
        write_fmt(value);
    }
}

// TODO remove
/// Implementation detail
pub fn write_prim(s: &Str) {
    write(&[s.address as u8])
}

/// Implementation detail
pub fn write_u8(b: &u8) {
    write(&[*b])
}

/// Implementation detail
pub fn write_u16(b: &u16) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_u24(b: &u32) {
    write(&b.to_le_bytes()[..3])
}

/// Implementation detail
pub fn write_u32(b: &u32) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_u64(b: &u64) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_u128(b: &u128) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_usize(b: &usize) {
    write(&b.to_le_bytes())
}

/// Implementation detail
pub fn write_f32(b: &f32) {
    write(&f32::to_bits(*b).to_le_bytes())
}

/// Implementation detail
pub fn write_f64(b: &f64) {
    write(&f64::to_bits(*b).to_le_bytes())
}

pub fn write_str(s: &str) {
    write_usize(&s.len());
    write(s.as_bytes());
}

pub fn write_slice(s: &[u8]) {
    write_usize(&s.len());
    write(s);
}

// NOTE: This is passed `&[u8; N]` – it's just coerced to a slice.
pub fn write_u8_array(a: &[u8]) {
    write(a);
}

// NOTE: This is passed `&[u8; N]` – it's just coerced to a slice.
pub fn write_fmt_array(a: &[impl Format]) {
    for value in a {
        write_fmt(value);
    }
}

/// Implementation detail
pub fn write_tag(tag: &u16) {
    write(&tag.to_le_bytes())
}

/// Implementation detail
pub fn write_istr(s: &Str) {
    write(&s.address.to_le_bytes())
}

/// Implementation detail
pub fn write_bool(b: &bool) {
    write_u8(&(*b as u8));
}

/// Implementation detail
pub fn write_debug(val: &dyn core::fmt::Debug) {
    core::write!(FmtWrite, "{:?}", val).ok();
    write(&[0xff]);
}

/// Implementation detail
pub fn write_display(val: &dyn core::fmt::Display) {
    core::write!(FmtWrite, "{}", val).ok();
    write(&[0xff]);
}

#[inline(never)]
pub fn write_header(s: &Str) {
    write_istr(s);
    timestamp(Formatter::new());
}
