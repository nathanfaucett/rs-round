use core::intrinsics::{
    floorf32, floorf64,
    ceilf32, ceilf64,
    roundf32, roundf64
};


pub trait Round {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
}

macro_rules! trait_round {
    ($t:ident) => (
        impl Round for $t {
            #[inline(always)]
            fn floor(self) -> $t { self }
            #[inline(always)]
            fn ceil(self) -> $t { self }
            #[inline(always)]
            fn round(self) -> $t { self }
        }
    );
}

trait_round!(usize);
trait_round!(u8);
trait_round!(u16);
trait_round!(u32);
trait_round!(u64);

trait_round!(isize);
trait_round!(i8);
trait_round!(i16);
trait_round!(i32);
trait_round!(i64);

impl Round for f32 {
    #[inline(always)]
    fn floor(self) -> f32 { unsafe { floorf32(self) } }
    #[inline(always)]
    fn ceil(self) -> f32 { unsafe { ceilf32(self) } }
    #[inline(always)]
    fn round(self) -> f32 { unsafe { roundf32(self) } }
}
impl Round for f64 {
    #[inline(always)]
    fn floor(self) -> f64 { unsafe { floorf64(self) } }
    #[inline(always)]
    fn ceil(self) -> f64 { unsafe { ceilf64(self) } }
    #[inline(always)]
    fn round(self) -> f64 { unsafe { roundf64(self) } }
}

#[test]
fn test_round() {
    assert_eq!((1.2f32).round(), 1f32);
    assert_eq!((1.8f32).round(), 2f32);
    assert_eq!((-1.2f32).round(), -1f32);
    assert_eq!((-1.8f32).round(), -2f32);

    assert_eq!((1.2f64).round(), 1f64);
    assert_eq!((1.8f64).round(), 2f64);
    assert_eq!((-1.2f64).round(), -1f64);
    assert_eq!((-1.8f64).round(), -2f64);

    assert_eq!((2).round(), 2);
}
