use super::MaxSize;
use core::mem::size_of;
use core::marker::PhantomData;

trait Primitive {}

macro_rules! impl_Primitive_for_primitive {
    ($($Ts:ty),+) => {
        $(impl Primitive for $Ts {})+
    };
}

// Remove this macro and support NonZero once `ZeroablePrimitive` or similar is approved
impl_Primitive_for_primitive!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool, usize, isize);

impl<T: Primitive> MaxSize for T {
    const ENCODED_MAX_SIZE: usize = size_of::<T>();
}

impl<T: MaxSize, const N: usize> MaxSize for [T; N] {
    const ENCODED_MAX_SIZE: usize = N * T::ENCODED_MAX_SIZE;
}

impl<T> MaxSize for PhantomData<T> {
    const ENCODED_MAX_SIZE: usize = 0;
}

macro_rules! impl_MaxSize_for_tuple {
    ($($Ts:tt),+) => {
        impl <$($Ts: MaxSize),+> MaxSize for ($($Ts),+){
            const ENCODED_MAX_SIZE: usize = $($Ts::ENCODED_MAX_SIZE+)+ 0;
        }
    };
}

macro_rules! impl_pyramid_MaxSize_for_tuple {
    ($T1:tt, $($Ts:tt),+) => {
        impl_MaxSize_for_tuple!($T1, $($Ts),+);
        impl_pyramid_MaxSize_for_tuple!($($Ts),*);
    };
    ($T1:tt) => {}
}

impl_pyramid_MaxSize_for_tuple!(
    T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
);