use super::{BorrowDecode, BorrowDecoder, Decode, Decoder};
use crate::error::DecodeError;

impl<'de, A> BorrowDecode<'de> for (A,)
where
    A: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((A::borrow_decode(decoder)?,))
    }
}
impl<A> Decode for (A,)
where
    A: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((A::decode(decoder)?,))
    }
}
impl<'de, A, B> BorrowDecode<'de> for (A, B)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((A::borrow_decode(decoder)?, B::borrow_decode(decoder)?))
    }
}
impl<A, B> Decode for (A, B)
where
    A: Decode,
    B: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((A::decode(decoder)?, B::decode(decoder)?))
    }
}
impl<'de, A, B, C> BorrowDecode<'de> for (A, B, C)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C> Decode for (A, B, C)
where
    A: Decode,
    B: Decode,
    C: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D> BorrowDecode<'de> for (A, B, C, D)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
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
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E> BorrowDecode<'de> for (A, B, C, D, E)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
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
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F> BorrowDecode<'de> for (A, B, C, D, E, F)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
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
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G> BorrowDecode<'de> for (A, B, C, D, E, F, G)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
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
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H> BorrowDecode<'de> for (A, B, C, D, E, F, G, H)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
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
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I> BorrowDecode<'de> for (A, B, C, D, E, F, G, H, I)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I> Decode for (A, B, C, D, E, F, G, H, I)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J> BorrowDecode<'de> for (A, B, C, D, E, F, G, H, I, J)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J> Decode for (A, B, C, D, E, F, G, H, I, J)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K> BorrowDecode<'de> for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K> Decode for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K, L> BorrowDecode<'de>
    for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
    L: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
            L::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L> Decode for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
    L: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
            L::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K, L, M> BorrowDecode<'de>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
    L: BorrowDecode<'de>,
    M: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
            L::borrow_decode(decoder)?,
            M::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M> Decode for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
    L: Decode,
    M: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
            L::decode(decoder)?,
            M::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K, L, M, N> BorrowDecode<'de>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
    L: BorrowDecode<'de>,
    M: BorrowDecode<'de>,
    N: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
            L::borrow_decode(decoder)?,
            M::borrow_decode(decoder)?,
            N::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> Decode for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
    L: Decode,
    M: Decode,
    N: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
            L::decode(decoder)?,
            M::decode(decoder)?,
            N::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> BorrowDecode<'de>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
    L: BorrowDecode<'de>,
    M: BorrowDecode<'de>,
    N: BorrowDecode<'de>,
    O: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
            L::borrow_decode(decoder)?,
            M::borrow_decode(decoder)?,
            N::borrow_decode(decoder)?,
            O::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> Decode
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
    L: Decode,
    M: Decode,
    N: Decode,
    O: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
            L::decode(decoder)?,
            M::decode(decoder)?,
            N::decode(decoder)?,
            O::decode(decoder)?,
        ))
    }
}
impl<'de, A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> BorrowDecode<'de>
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    A: BorrowDecode<'de>,
    B: BorrowDecode<'de>,
    C: BorrowDecode<'de>,
    D: BorrowDecode<'de>,
    E: BorrowDecode<'de>,
    F: BorrowDecode<'de>,
    G: BorrowDecode<'de>,
    H: BorrowDecode<'de>,
    I: BorrowDecode<'de>,
    J: BorrowDecode<'de>,
    K: BorrowDecode<'de>,
    L: BorrowDecode<'de>,
    M: BorrowDecode<'de>,
    N: BorrowDecode<'de>,
    O: BorrowDecode<'de>,
    P: BorrowDecode<'de>,
{
    fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
        Ok((
            A::borrow_decode(decoder)?,
            B::borrow_decode(decoder)?,
            C::borrow_decode(decoder)?,
            D::borrow_decode(decoder)?,
            E::borrow_decode(decoder)?,
            F::borrow_decode(decoder)?,
            G::borrow_decode(decoder)?,
            H::borrow_decode(decoder)?,
            I::borrow_decode(decoder)?,
            J::borrow_decode(decoder)?,
            K::borrow_decode(decoder)?,
            L::borrow_decode(decoder)?,
            M::borrow_decode(decoder)?,
            N::borrow_decode(decoder)?,
            O::borrow_decode(decoder)?,
            P::borrow_decode(decoder)?,
        ))
    }
}
impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> Decode
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    A: Decode,
    B: Decode,
    C: Decode,
    D: Decode,
    E: Decode,
    F: Decode,
    G: Decode,
    H: Decode,
    I: Decode,
    J: Decode,
    K: Decode,
    L: Decode,
    M: Decode,
    N: Decode,
    O: Decode,
    P: Decode,
{
    fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
        Ok((
            A::decode(decoder)?,
            B::decode(decoder)?,
            C::decode(decoder)?,
            D::decode(decoder)?,
            E::decode(decoder)?,
            F::decode(decoder)?,
            G::decode(decoder)?,
            H::decode(decoder)?,
            I::decode(decoder)?,
            J::decode(decoder)?,
            K::decode(decoder)?,
            L::decode(decoder)?,
            M::decode(decoder)?,
            N::decode(decoder)?,
            O::decode(decoder)?,
            P::decode(decoder)?,
        ))
    }
}

// For re-generating the above code
// - uncomment the code below
// - run `cargo expand > expanded.rs`
// - find `mod impl_tuples` and copy that code above
// macro_rules! impl_tuple {
//     () => {};
//     ($first:ident $(, $extra:ident)*) => {
//         impl<'de, $first $(, $extra)*> BorrowDecode<'de> for ($first, $($extra, )*)
//         where
//             $first: BorrowDecode<'de>,
//         $(
//             $extra : BorrowDecode<'de>,
//         )*
//          {
//             fn borrow_decode<BD: BorrowDecoder<'de>>(decoder: &mut BD) -> Result<Self, DecodeError> {
//                 Ok((
//                     $first::borrow_decode(decoder)?,
//                     $($extra :: borrow_decode(decoder)?, )*
//                 ))
//             }
//         }
//
//         impl<$first $(, $extra)*> Decode for ($first, $($extra, )*)
//         where
//             $first: Decode,
//         $(
//             $extra : Decode,
//         )*
//         {
//             fn decode<DE: Decoder>(decoder: &mut DE) -> Result<Self, DecodeError> {
//                 Ok((
//                     $first::decode(decoder)?,
//                     $($extra :: decode(decoder)?, )*
//                 ))
//             }
//         }
//     }
// }
//
// impl_tuple!(A);
// impl_tuple!(A, B);
// impl_tuple!(A, B, C);
// impl_tuple!(A, B, C, D);
// impl_tuple!(A, B, C, D, E);
// impl_tuple!(A, B, C, D, E, F);
// impl_tuple!(A, B, C, D, E, F, G);
// impl_tuple!(A, B, C, D, E, F, G, H);
// impl_tuple!(A, B, C, D, E, F, G, H, I);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
// impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P;
