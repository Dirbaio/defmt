use crate as defmt;
use defmt_macros::internp;

use crate::{export, Format, Formatter, Str};

impl Format for i8 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=i8}");
        export::write_tag(&t);
        export::write_u8(&(*self as u8));
    }
}

impl Format for i16 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=i16}");
        export::write_tag(&t);
        export::write_u16(&(*self as u16))
    }
}

impl Format for i32 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=i32}");
        export::write_tag(&t);
        export::write_i32(self);
    }
}

impl Format for i64 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=i64}");
        export::write_tag(&t);
        export::write_i64(self);
    }
}

impl Format for i128 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=i128}");
        export::write_tag(&t);
        export::write_i128(self);
    }
}

impl Format for isize {
    fn format(&self, _: Formatter) {
        let t = internp!("{=isize}");
        export::write_tag(&t);
        export::write_isize(self);
    }
}

impl Format for u8 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=u8}");
        export::write_tag(&t);
        export::write_u8(self)
    }
}

impl Format for u16 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=u16}");
        export::write_tag(&t);
        export::write_u16(self);
    }
}

impl Format for u32 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=u32}");
        export::write_tag(&t);
        export::write_u32(self);
    }
}

impl Format for u64 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=u64}");
        export::write_tag(&t);
        export::write_u64(self);
    }
}

impl Format for u128 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=u128}");
        export::write_tag(&t);
        export::write_u128(self);
    }
}

impl Format for usize {
    fn format(&self, _: Formatter) {
        let t = internp!("{=usize}");
        export::write_tag(&t);
        export::write_usize(self);
    }
}

impl Format for f32 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=f32}");
        export::write_tag(&t);
        export::write_f32(self);
    }
}

impl Format for f64 {
    fn format(&self, _: Formatter) {
        let t = internp!("{=f64}");
        export::write_tag(&t);
        export::write_f64(self);
    }
}

impl Format for str {
    fn format(&self, _: Formatter) {
        let t = str_tag();
        export::write_tag(&t);
        export::write_str(self);
    }
}

pub(crate) fn str_tag() -> u16 {
    internp!("{=str}")
}

impl Format for Str {
    fn format(&self, _: Formatter) {
        let t = internp!("{=istr}");
        export::write_tag(&t);
        export::write_istr(self);
    }
}

impl Format for char {
    fn format(&self, _: Formatter) {
        let t = internp!("{=char}");
        export::write_tag(&t);
        export::write_u32(&(*self as u32));
    }
}

impl<T> Format for [T]
where
    T: Format,
{
    fn format(&self, _: Formatter) {
        let t = internp!("{=[?]}");
        export::write_tag(&t);
        export::write_fmt_slice(self)
    }
}

impl<T> Format for &'_ T
where
    T: Format + ?Sized,
{
    fn format(&self, fmt: Formatter) {
        T::format(self, fmt)
    }
}

impl<T> Format for &'_ mut T
where
    T: Format + ?Sized,
{
    fn format(&self, fmt: Formatter) {
        T::format(self, fmt)
    }
}

impl Format for bool {
    fn format(&self, _: Formatter) {
        let t = internp!("{=bool}");
        export::write_tag(&t);
        export::write_bool(self);
    }
}

macro_rules! arrays {
    ( $($len:literal $fmt:literal,)+ ) => { $(
        impl<T> Format for [T; $len]
        where
            T: Format
        {
            fn format(&self, _: Formatter) {
                    let t = internp!($fmt);
                    export::write_tag(&t);
                export::write_fmt_array(self);
            }
        }
    )+ };
}

arrays! {
    0 "{=[?;0]}",
    1 "{=[?;1]}",
    2 "{=[?;2]}",
    3 "{=[?;3]}",
    4 "{=[?;4]}",
    5 "{=[?;5]}",
    6 "{=[?;6]}",
    7 "{=[?;7]}",
    8 "{=[?;8]}",
    9 "{=[?;9]}",
    10 "{=[?;10]}",
    11 "{=[?;11]}",
    12 "{=[?;12]}",
    13 "{=[?;13]}",
    14 "{=[?;14]}",
    15 "{=[?;15]}",
    16 "{=[?;16]}",
    17 "{=[?;17]}",
    18 "{=[?;18]}",
    19 "{=[?;19]}",
    20 "{=[?;20]}",
    21 "{=[?;21]}",
    22 "{=[?;22]}",
    23 "{=[?;23]}",
    24 "{=[?;24]}",
    25 "{=[?;25]}",
    26 "{=[?;26]}",
    27 "{=[?;27]}",
    28 "{=[?;28]}",
    29 "{=[?;29]}",
    30 "{=[?;30]}",
    31 "{=[?;31]}",
    32 "{=[?;32]}",
    64 "{=[?;64]}",
    128 "{=[?;128]}",
    256 "{=[?;256]}",
    512 "{=[?;512]}",
    1024 "{=[?;1024]}",
    2048 "{=[?;2048]}",
    4096 "{=[?;4096]}",
    8192 "{=[?;8192]}",
    16384 "{=[?;16384]}",
    32768 "{=[?;32768]}",
    65536 "{=[?;65536]}",
    131072 "{=[?;131072]}",
    262144 "{=[?;262144]}",
    524288 "{=[?;524288]}",
    1048576 "{=[?;1048576]}",
    2097152 "{=[?;2097152]}",
    4194304 "{=[?;4194304]}",
    8388608 "{=[?;8388608]}",
    16777216 "{=[?;16777216]}",
    33554432 "{=[?;33554432]}",
    67108864 "{=[?;67108864]}",
    134217728 "{=[?;134217728]}",
    268435456 "{=[?;268435456]}",
    536870912 "{=[?;536870912]}",
    1073741824 "{=[?;1073741824]}",
    100 "{=[?;100]}",
    1000 "{=[?;1000]}",
    10000 "{=[?;10000]}",
    100000 "{=[?;100000]}",
    1000000 "{=[?;1000000]}",
    10000000 "{=[?;10000000]}",
    100000000 "{=[?;100000000]}",
    1000000000 "{=[?;1000000000]}",
}

impl<T> Format for Option<T>
where
    T: Format,
{
    fn format(&self, f: Formatter) {
        let t = internp!("None|Some({=?})");
        export::write_tag(&t);
        match self {
            None => export::write_u8(&0),
            Some(x) => {
                export::write_u8(&1);
                x.format(f)
            }
        }
    }
}

impl<T, E> Format for Result<T, E>
where
    T: Format,
    E: Format,
{
    fn format(&self, f: Formatter) {
        let t = internp!("Err({=?})|Ok({=?})");
        export::write_tag(&t);
        match self {
            Err(e) => {
                export::write_u8(&0);
                e.format(f)
            }
            Ok(x) => {
                export::write_u8(&1);
                x.format(f)
            }
        }
    }
}

impl Format for () {
    fn format(&self, _: Formatter) {
        let t = internp!("()");
        export::write_tag(&t);
    }
}

impl<T> Format for core::marker::PhantomData<T> {
    fn format(&self, _: Formatter) {
        let t = internp!("PhantomData");
        export::write_tag(&t);
    }
}

macro_rules! tuple {
    ( $format:expr, ($($name:ident),+) ) => (
        impl<$($name:Format),+> Format for ($($name,)+) where last_type!($($name,)+): ?Sized {
            #[allow(non_snake_case, unused_assignments)]
            fn format(&self, _: Formatter) {
                    let t = internp!($format);
                    export::write_tag(&t);

                let ($(ref $name,)+) = *self;
                $(
                    $name.format(Formatter::new());
                )+
            }
        }
    )
}

macro_rules! last_type {
    ($a:ident,) => { $a };
    ($a:ident, $($rest_a:ident,)+) => { last_type!($($rest_a,)+) };
}

tuple! { "({=?})", (T0) }
tuple! { "({=?}, {=?})", (T0, T1) }
tuple! { "({=?}, {=?}, {=?})", (T0, T1, T2) }
tuple! { "({=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6, T7) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6, T7, T8) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10) }
tuple! { "({=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?}, {=?})", (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11) }

#[cfg(feature = "alloc")]
mod if_alloc {
    use crate::{Format, Formatter};

    impl<T> Format for alloc::boxed::Box<T>
    where
        T: ?Sized + Format,
    {
        fn format(&self, f: Formatter) {
            T::format(&*self, f)
        }
    }

    impl<T> Format for alloc::rc::Rc<T>
    where
        T: ?Sized + Format,
    {
        fn format(&self, f: Formatter) {
            T::format(&*self, f)
        }
    }

    #[cfg(not(no_cas))]
    impl<T> Format for alloc::sync::Arc<T>
    where
        T: ?Sized + Format,
    {
        fn format(&self, f: Formatter) {
            T::format(&*self, f)
        }
    }

    impl<T> Format for alloc::vec::Vec<T>
    where
        T: Format,
    {
        fn format(&self, f: Formatter) {
            self.as_slice().format(f)
        }
    }

    impl Format for alloc::string::String {
        fn format(&self, f: Formatter) {
            self.as_str().format(f)
        }
    }
}

impl Format for core::convert::Infallible {
    #[inline]
    fn format(&self, _: Formatter) {
        // type cannot be instantiated so nothing to do here
        match *self {}
    }
}

impl Format for core::time::Duration {
    fn format(&self, fmt: Formatter) {
        crate::write!(
            fmt,
            "Duration {{ secs: {=u64}, nanos: {=u32} }}",
            self.as_secs(),
            self.subsec_nanos(),
        )
    }
}
