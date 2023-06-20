#[cfg(feature = "alloc")]
mod impl_alloc;
#[cfg(feature = "alloc")]
pub use self::impl_alloc::*;

#[cfg(feature = "std")]
mod impl_std;
#[cfg(feature = "std")]
pub use self::impl_std::*;

#[cfg(feature = "embedded-io")]
mod impl_embedded_io;
#[cfg(feature = "embedded-io")]
pub use self::impl_embedded_io::*;

#[cfg(feature = "derive")]
mod derive;
#[cfg(feature = "derive")]
pub use self::derive::*;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub mod serde;
