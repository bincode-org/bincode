use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::DecodeError;

macro_rules! impl_tuple {
    () => {};
    ($first:ident, $($extra:ident, )*) => {
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


        impl_tuple!($($extra, )*);
    }
}

impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P,);

// impl<'de, A> BorrowDecode<'de> for (A, )
// where A: BorrowDecode<'de> {
//     fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
//         Ok((A::borrow_decode(&mut decoder)?, ))
//     }
// }
// impl<A> Decode for (A,)
// where
//     A: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((A::decode(&mut decoder)?,))
//     }
// }

// impl<'de, A, B> BorrowDecode<'de> for (A, B)
// where A: BorrowDecode<'de>, B: BorrowDecode<'de> {
//     fn borrow_decode<D: BorrowDecoder<'de>>(decoder: &mut D) -> Result<Self, DecodeError> {
//         Ok((A::borrow_decode(&mut decoder)?, B::borrow_decode(&mut decoder)?))
//     }
// }
// impl<A, B> Decode for (A, B)
// where
//     A: Decode,
//     B: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((A::decode(&mut decoder)?, B::decode(&mut decoder)?))
//     }
// }

// impl<A, B, C> Decode for (A, B, C)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//         ))
//     }
// }

// impl<A, B, C, D> Decode for (A, B, C, D)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
//     D: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//             D::decode(&mut decoder)?,
//         ))
//     }
// }

// impl<A, B, C, D, E> Decode for (A, B, C, D, E)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
//     D: Decode,
//     E: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//             D::decode(&mut decoder)?,
//             E::decode(&mut decoder)?,
//         ))
//     }
// }

// impl<A, B, C, D, E, F> Decode for (A, B, C, D, E, F)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
//     D: Decode,
//     E: Decode,
//     F: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//             D::decode(&mut decoder)?,
//             E::decode(&mut decoder)?,
//             F::decode(&mut decoder)?,
//         ))
//     }
// }

// impl<A, B, C, D, E, F, G> Decode for (A, B, C, D, E, F, G)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
//     D: Decode,
//     E: Decode,
//     F: Decode,
//     G: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//             D::decode(&mut decoder)?,
//             E::decode(&mut decoder)?,
//             F::decode(&mut decoder)?,
//             G::decode(&mut decoder)?,
//         ))
//     }
// }

// impl<A, B, C, D, E, F, G, H> Decode for (A, B, C, D, E, F, G, H)
// where
//     A: Decode,
//     B: Decode,
//     C: Decode,
//     D: Decode,
//     E: Decode,
//     F: Decode,
//     G: Decode,
//     H: Decode,
// {
//     fn decode<_D: Decoder>(mut decoder: &mut _D) -> Result<Self, DecodeError> {
//         Ok((
//             A::decode(&mut decoder)?,
//             B::decode(&mut decoder)?,
//             C::decode(&mut decoder)?,
//             D::decode(&mut decoder)?,
//             E::decode(&mut decoder)?,
//             F::decode(&mut decoder)?,
//             G::decode(&mut decoder)?,
//             H::decode(&mut decoder)?,
//         ))
//     }
// }
