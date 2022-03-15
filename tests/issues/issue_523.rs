#![cfg(all(feature = "derive", feature = "std"))]

extern crate std;

use bincode::Encode;
use std::borrow::Cow;

#[derive(Clone, Encode)]
pub struct Foo<'a>(Cow<'a, str>);
