use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::DecodeError;

macro_rules! impl_tuple {
    () => {};
    ($first:ident $(, $extra:ident)*) => {
        impl<'de, $first $(, $extra)*> BorrowDecode<'de> for ($first, $($extra, )*)
        where
            $first: BorrowDecode<'de>,
        $(
            $extra : BorrowDecode<'de>,
        )*
         {
            fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
                Ok((
                    $first::borrow_decode(decoder)?,
                    $($extra :: borrow_decode(decoder)?, )*
                ))
            }
        }

        impl<$first $(, $extra)*> Decode for ($first, $($extra, )*)
        where
            $first: Decode,
        $(
            $extra : Decode,
        )*
        {
            fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
                Ok((
                    $first::decode(decoder)?,
                    $($extra :: decode(decoder)?, )*
                ))
            }
        }
    }
}

impl_tuple!(A);
impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
