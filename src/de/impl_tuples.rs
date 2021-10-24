use super::{Decode, Decoder};
use crate::error::DecodeError;

impl<A> Decode for (A,)
where
    A: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((A::decode(&mut decoder)?,))
    }
}

impl<A, B> Decode for (A, B)
where
    A: Decode,
    B: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((A::decode(&mut decoder)?, B::decode(&mut decoder)?))
    }
}

impl<A, B, C> Decode for (A, B, C)
where
    A: Decode,
    B: Decode,
    C: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D> Decode for (A, B, C, D)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E> Decode for (A, B, C, D, E)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
            E::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E, F> Decode for (A, B, C, D, E, F)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
            E::decode(&mut decoder)?,
            F::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E, F, G> Decode for (A, B, C, D, E, F, G)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
            E::decode(&mut decoder)?,
            F::decode(&mut decoder)?,
            G::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E, F, G, H> Decode for (A, B, C, D, E, F, G, H)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
{
    fn decode<_D: Decoder>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
            E::decode(&mut decoder)?,
            F::decode(&mut decoder)?,
            G::decode(&mut decoder)?,
            H::decode(&mut decoder)?,
        ))
    }
}
