#![cfg(feature = "alloc")]

extern crate alloc;

mod utils;

use alloc::borrow::Cow;
use alloc::rc::Rc;
use alloc::sync::Arc;
use utils::the_same;

struct Foo {
    pub a: u32,
    pub b: u32,
}

impl bincode::enc::Encodeable for Foo {
    fn encode<E: bincode::enc::Encode>(
        &self,
        mut encoder: E,
    ) -> Result<(), bincode::error::EncodeError> {
        self.a.encode(&mut encoder)?;
        self.b.encode(&mut encoder)?;
        Ok(())
    }
}

impl bincode::de::Decodable for Foo {
    fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
        Ok(Self {
            a: bincode::de::Decodable::decode(&mut decoder)?,
            b: bincode::de::Decodable::decode(&mut decoder)?,
        })
    }
}

#[test]
fn test_vec() {
    let vec = bincode::encode_to_vec(Foo { a: 5, b: 10 }).unwrap();
    assert_eq!(vec, &[5, 10]);

    let foo: Foo = bincode::decode(&vec).unwrap();
    assert_eq!(foo.a, 5);
    assert_eq!(foo.b, 10);
}

#[test]
fn test_alloc_commons() {
    the_same::<Vec<u32>>(vec![1, 2, 3, 4, 5]);
    the_same(Box::<u32>::new(5));
    the_same(Box::<[u32]>::from(vec![1, 2, 3, 4, 5]));
    the_same(Cow::<u32>::Owned(5));
    the_same(Cow::<u32>::Borrowed(&5));
    the_same(Rc::<u32>::new(5));
    the_same(Arc::<u32>::new(5));
}
