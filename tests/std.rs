#![cfg(feature = "std")]

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
fn test_std_cursor() {
    let mut cursor = std::io::Cursor::<&[u8]>::new(&[5, 10]);
    let foo: Foo = bincode::decode_from(&mut cursor).unwrap();

    assert_eq!(foo.a, 5);
    assert_eq!(foo.b, 10);
}

#[test]
fn test_std_file() {
    use std::io::{Seek, SeekFrom};

    let mut file = tempfile::tempfile().expect("Could not create temp file");

    let bytes_written = bincode::encode_into_write(Foo { a: 30, b: 50 }, &mut file).unwrap();
    assert_eq!(bytes_written, 2);
    file.seek(SeekFrom::Start(0)).unwrap();

    let foo: Foo = bincode::decode_from(&mut file).unwrap();

    assert_eq!(foo.a, 30);
    assert_eq!(foo.b, 50);
}
