use super::{Encode, Encodeable};
use crate::error::EncodeError;

impl<A> Encodeable for (A,)
where
    A: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B> Encodeable for (A, B)
where
    A: Encodeable,
    B: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C> Encodeable for (A, B, C)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D> Encodeable for (A, B, C, D)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
    D: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E> Encodeable for (A, B, C, D, E)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
    D: Encodeable,
    E: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F> Encodeable for (A, B, C, D, E, F)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
    D: Encodeable,
    E: Encodeable,
    F: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F, G> Encodeable for (A, B, C, D, E, F, G)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
    D: Encodeable,
    E: Encodeable,
    F: Encodeable,
    G: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        self.6.encode(&mut encoder)?;
        Ok(())
    }
}

impl<A, B, C, D, E, F, G, H> Encodeable for (A, B, C, D, E, F, G, H)
where
    A: Encodeable,
    B: Encodeable,
    C: Encodeable,
    D: Encodeable,
    E: Encodeable,
    F: Encodeable,
    G: Encodeable,
    H: Encodeable,
{
    fn encode<_E: Encode>(&self, mut encoder: _E) -> Result<(), EncodeError> {
        self.0.encode(&mut encoder)?;
        self.1.encode(&mut encoder)?;
        self.2.encode(&mut encoder)?;
        self.3.encode(&mut encoder)?;
        self.4.encode(&mut encoder)?;
        self.5.encode(&mut encoder)?;
        self.6.encode(&mut encoder)?;
        self.7.encode(&mut encoder)?;
        Ok(())
    }
}
