//! Size determination structs and traits.

mod impl_tuples;
mod impls;

use crate::config::Config;
use crate::error::EncodeError;

/// Any source whose size when encoded can be determined.
///
/// This trait should be implemented for all types that you want to be able to determine the encoded size ahead of actual encoding.
///
/// # Implementing this trait manually
///
/// If you want to implement this trait for your type, the easiest way is to add a `#[derive(bincode::EncodedSize)]`, build and check your `target/generated/bincode/` folder. This should generate a `<Struct name>_EncodedSize.rs` file.
///
/// For this struct:
///
/// ```
/// struct Entity {
///     pub x: f32,
///     pub y: f32,
/// }
/// ```
/// It will look something like:
///
/// ```
/// # struct Entity {
/// #     pub x: f32,
/// #     pub y: f32,
/// # }
/// impl bincode::EncodedSize for Entity {
///     fn encoded_size<C: bincode::config::Config>(
///         &self,
///     ) -> core::result::Result<usize, bincode::error::EncodeError> {
///         let mut __encoded_size = 0;
///         __encoded_size += bincode::EncodedSize::encoded_size::<C>(&self.x)?;
///         __encoded_size += bincode::EncodedSize::encoded_size::<C>(&self.y)?;
///         Ok(__encoded_size)
///     }
/// }
/// ```
///
/// From here you can add/remove fields, or add custom logic.
///
/// # Interior Mutability
///
/// Types with interior mutability may be mutated between calls to `encoded_size` and one of the `encode` methods.  If this happens, the encoded size may change.  You must ensure that your encoded values are either not mutated between calls to `encoded_size` and `encode`, or handle the case where the actual encoded size is large than the value that `encoded_size` returns.
pub trait EncodedSize {
    /// Determine the encoded size of a given type.
    fn encoded_size<C: Config>(&self) -> Result<usize, EncodeError>;
}

/// Returns the size of the encoded length of any slice, container, etc.
#[inline]
pub(crate) fn size_slice_len<C: Config>(len: usize) -> Result<usize, EncodeError> {
    (len as u64).encoded_size::<C>()
}
