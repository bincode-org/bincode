pub trait Sealed {}

impl<'a, T> Sealed for &'a mut T where T: Sealed {}
