use std::{ptr, rc::Rc, sync::Arc};

use crate::Color;

pub trait Data: 'static + Clone {
    fn same(&self, other: &Self) -> bool;
}

// impl<T> Data for State<T> {

//     fn count(&self) -> usize {
//         self.version()
//     }
// }

macro_rules! impl_data_simple {
    ($t:ty) => {
        impl Data for $t {
            fn same(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

impl_data_simple!(i8);
impl_data_simple!(i16);
impl_data_simple!(i32);
impl_data_simple!(i64);
impl_data_simple!(i128);
impl_data_simple!(isize);
impl_data_simple!(u8);
impl_data_simple!(u16);
impl_data_simple!(u32);
impl_data_simple!(u64);
impl_data_simple!(u128);
impl_data_simple!(usize);
impl_data_simple!(char);
impl_data_simple!(bool);
impl_data_simple!(std::num::NonZeroI8);
impl_data_simple!(std::num::NonZeroI16);
impl_data_simple!(std::num::NonZeroI32);
impl_data_simple!(std::num::NonZeroI64);
impl_data_simple!(std::num::NonZeroI128);
impl_data_simple!(std::num::NonZeroIsize);
impl_data_simple!(std::num::NonZeroU8);
impl_data_simple!(std::num::NonZeroU16);
impl_data_simple!(std::num::NonZeroU32);
impl_data_simple!(std::num::NonZeroU64);
impl_data_simple!(std::num::NonZeroU128);
impl_data_simple!(std::num::NonZeroUsize);
impl_data_simple!(std::time::SystemTime);
impl_data_simple!(std::time::Instant);
impl_data_simple!(std::time::Duration);
impl_data_simple!(std::io::ErrorKind);
impl_data_simple!(std::net::Ipv4Addr);
impl_data_simple!(std::net::Ipv6Addr);
impl_data_simple!(std::net::SocketAddrV4);
impl_data_simple!(std::net::SocketAddrV6);
impl_data_simple!(std::net::IpAddr);
impl_data_simple!(std::net::SocketAddr);
impl_data_simple!(std::ops::RangeFull);

impl_data_simple!(String);

impl Data for &'static str {
    fn same(&self, other: &Self) -> bool {
        ptr::eq(*self, *other)
    }
}

impl Data for f32 {
    fn same(&self, other: &Self) -> bool {
        self.to_bits() == other.to_bits()
    }
}

impl Data for f64 {
    fn same(&self, other: &Self) -> bool {
        self.to_bits() == other.to_bits()
    }
}

impl<T: ?Sized + 'static> Data for Arc<T> {
    fn same(&self, other: &Self) -> bool {
        Arc::ptr_eq(self, other)
    }
}

impl<T: ?Sized + 'static> Data for std::sync::Weak<T> {
    fn same(&self, other: &Self) -> bool {
        std::sync::Weak::ptr_eq(self, other)
    }
}

impl<T: ?Sized + 'static> Data for Rc<T> {
    fn same(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}

impl<T: ?Sized + 'static> Data for std::rc::Weak<T> {
    fn same(&self, other: &Self) -> bool {
        std::rc::Weak::ptr_eq(self, other)
    }
}

impl<T: Data> Data for Option<T> {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Some(a), Some(b)) => a.same(b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl<T: Data, U: Data> Data for Result<T, U> {
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Ok(a), Ok(b)) => a.same(b),
            (Err(a), Err(b)) => a.same(b),
            _ => false,
        }
    }
}

impl Data for () {
    fn same(&self, _other: &Self) -> bool {
        true
    }
}

impl<T0: Data> Data for (T0,) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl<T0: Data, T1: Data> Data for (T0, T1) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0) && self.1.same(&other.1)
    }
}

impl<T0: Data, T1: Data, T2: Data> Data for (T0, T1, T2) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0) && self.1.same(&other.1) && self.2.same(&other.2)
    }
}

impl<T0: Data, T1: Data, T2: Data, T3: Data> Data for (T0, T1, T2, T3) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
            && self.1.same(&other.1)
            && self.2.same(&other.2)
            && self.3.same(&other.3)
    }
}

impl<T0: Data, T1: Data, T2: Data, T3: Data, T4: Data> Data for (T0, T1, T2, T3, T4) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
            && self.1.same(&other.1)
            && self.2.same(&other.2)
            && self.3.same(&other.3)
            && self.4.same(&other.4)
    }
}

impl<T0: Data, T1: Data, T2: Data, T3: Data, T4: Data, T5: Data> Data for (T0, T1, T2, T3, T4, T5) {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
            && self.1.same(&other.1)
            && self.2.same(&other.2)
            && self.3.same(&other.3)
            && self.4.same(&other.4)
            && self.5.same(&other.5)
    }
}

impl<T: 'static + ?Sized> Data for std::marker::PhantomData<T> {
    fn same(&self, _other: &Self) -> bool {
        // zero-sized types
        true
    }
}

impl<T: 'static> Data for std::mem::Discriminant<T> {
    fn same(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl<T: 'static + ?Sized + Data> Data for std::mem::ManuallyDrop<T> {
    fn same(&self, other: &Self) -> bool {
        (&**self).same(&**other)
    }
}

impl<T: Data> Data for std::num::Wrapping<T> {
    fn same(&self, other: &Self) -> bool {
        self.0.same(&other.0)
    }
}

impl<T: Data> Data for std::ops::Range<T> {
    fn same(&self, other: &Self) -> bool {
        self.start.same(&other.start) && self.end.same(&other.end)
    }
}

impl<T: Data> Data for std::ops::RangeFrom<T> {
    fn same(&self, other: &Self) -> bool {
        self.start.same(&other.start)
    }
}

impl<T: Data> Data for std::ops::RangeInclusive<T> {
    fn same(&self, other: &Self) -> bool {
        self.start().same(other.start()) && self.end().same(other.end())
    }
}

impl<T: Data> Data for std::ops::RangeTo<T> {
    fn same(&self, other: &Self) -> bool {
        self.end.same(&other.end)
    }
}

impl<T: Data> Data for std::ops::RangeToInclusive<T> {
    fn same(&self, other: &Self) -> bool {
        self.end.same(&other.end)
    }
}

impl<T: Data> Data for std::ops::Bound<T> {
    fn same(&self, other: &Self) -> bool {
        use std::ops::Bound::*;
        match (self, other) {
            (Included(t1), Included(t2)) if t1.same(t2) => true,
            (Excluded(t1), Excluded(t2)) if t1.same(t2) => true,
            (Unbounded, Unbounded) => true,
            _ => false,
        }
    }
}

impl<T: Data> Data for Vec<T> {
    fn same(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }

        for (a, b) in self.iter().zip(other.iter()) {
            if !a.same(b) {
                return false;
            }
        }

        true
    }
}

impl Data for morphorm::Units {
    fn same(&self, other: &Self) -> bool {
        *self == *other
    }
}

impl Data for Color {
    fn same(&self, other: &Self) -> bool {
        *self == *other
    }
}

#[cfg(feature = "meadowlark")]
impl Data for rusty_daw_core::MusicalTime {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(feature = "meadowlark")]
impl Data for rusty_daw_core::SuperFrames {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}

#[cfg(feature = "meadowlark")]
impl Data for rusty_daw_core::Seconds {
    fn same(&self, other: &Self) -> bool {
        self == other
    }
}
