use super::EncodedSize;
use crate::config::Config;
use crate::error::EncodeError;

impl<A> EncodedSize for (A,)
where
    A: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        self.0.encoded_size::<_C>()
    }
}

impl<A, B> EncodedSize for (A, B)
where
    A: EncodedSize,
    B: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()? + self.1.encoded_size::<_C>()?)
    }
}

impl<A, B, C> EncodedSize for (A, B, C)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D> EncodedSize for (A, B, C, D)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E> EncodedSize for (A, B, C, D, E)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F> EncodedSize for (A, B, C, D, E, F)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G> EncodedSize for (A, B, C, D, E, F, G)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H> EncodedSize for (A, B, C, D, E, F, G, H)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I> EncodedSize for (A, B, C, D, E, F, G, H, I)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> EncodedSize for (A, B, C, D, E, F, G, H, I, J)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> EncodedSize for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> EncodedSize for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
    L: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?
            + self.11.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M> EncodedSize for (A, B, C, D, E, F, G, H, I, J, K, L, M)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
    L: EncodedSize,
    M: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?
            + self.11.encoded_size::<_C>()?
            + self.12.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N> EncodedSize
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
    L: EncodedSize,
    M: EncodedSize,
    N: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?
            + self.11.encoded_size::<_C>()?
            + self.12.encoded_size::<_C>()?
            + self.13.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O> EncodedSize
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
    L: EncodedSize,
    M: EncodedSize,
    N: EncodedSize,
    O: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?
            + self.11.encoded_size::<_C>()?
            + self.12.encoded_size::<_C>()?
            + self.13.encoded_size::<_C>()?
            + self.14.encoded_size::<_C>()?)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P> EncodedSize
    for (A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P)
where
    A: EncodedSize,
    B: EncodedSize,
    C: EncodedSize,
    D: EncodedSize,
    E: EncodedSize,
    F: EncodedSize,
    G: EncodedSize,
    H: EncodedSize,
    I: EncodedSize,
    J: EncodedSize,
    K: EncodedSize,
    L: EncodedSize,
    M: EncodedSize,
    N: EncodedSize,
    O: EncodedSize,
    P: EncodedSize,
{
    fn encoded_size<_C: Config>(&self) -> Result<usize, EncodeError> {
        Ok(self.0.encoded_size::<_C>()?
            + self.1.encoded_size::<_C>()?
            + self.2.encoded_size::<_C>()?
            + self.3.encoded_size::<_C>()?
            + self.4.encoded_size::<_C>()?
            + self.5.encoded_size::<_C>()?
            + self.6.encoded_size::<_C>()?
            + self.7.encoded_size::<_C>()?
            + self.8.encoded_size::<_C>()?
            + self.9.encoded_size::<_C>()?
            + self.10.encoded_size::<_C>()?
            + self.11.encoded_size::<_C>()?
            + self.12.encoded_size::<_C>()?
            + self.13.encoded_size::<_C>()?
            + self.14.encoded_size::<_C>()?
            + self.15.encoded_size::<_C>()?)
    }
}
