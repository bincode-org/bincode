use super::{Decodable, Decode};
use crate::error::DecodeError;

impl<A> Decodable for (A,)
where
    A: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((A::decode(&mut decoder)?,))
    }
}

impl<A, B> Decodable for (A, B)
where
    A: Decodable,
    B: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((A::decode(&mut decoder)?, B::decode(&mut decoder)?))
    }
}

impl<A, B, C> Decodable for (A, B, C)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D> Decodable for (A, B, C, D)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
    D: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E> Decodable for (A, B, C, D, E)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
    D: Decodable,
    E: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
        Ok((
            A::decode(&mut decoder)?,
            B::decode(&mut decoder)?,
            C::decode(&mut decoder)?,
            D::decode(&mut decoder)?,
            E::decode(&mut decoder)?,
        ))
    }
}

impl<A, B, C, D, E, F> Decodable for (A, B, C, D, E, F)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
    D: Decodable,
    E: Decodable,
    F: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
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

impl<A, B, C, D, E, F, G> Decodable for (A, B, C, D, E, F, G)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
    D: Decodable,
    E: Decodable,
    F: Decodable,
    G: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
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

impl<A, B, C, D, E, F, G, H> Decodable for (A, B, C, D, E, F, G, H)
where
    A: Decodable,
    B: Decodable,
    C: Decodable,
    D: Decodable,
    E: Decodable,
    F: Decodable,
    G: Decodable,
    H: Decodable,
{
    fn decode<_D: Decode>(mut decoder: _D) -> Result<Self, DecodeError> {
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
