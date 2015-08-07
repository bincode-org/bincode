#![feature(plugin, custom_derive)]
#![plugin(serde_macros)]

extern crate bincode;
extern crate rustc_serialize;
extern crate serde;

use std::fmt::Debug;
use std::collections::HashMap;
use std::ops::Deref;

use rustc_serialize::{Encoder, Decoder, Encodable, Decodable};

use bincode::{
    encode,
    decode,
    decode_from,
    encoded_size,
    DecodingError,
    DecodingResult,
    DeserializeError,
    DeserializeResult,
    RefBox,
    StrBox,
    SliceBox,
};

use bincode::SizeLimit::{self, Infinite, Bounded};

fn proxy_encode<V>(element: &V, size_limit: SizeLimit) -> Vec<u8>
    where V: Encodable + Decodable + serde::Serialize + serde::Deserialize + PartialEq + Debug + 'static
{
    let v1 = bincode::encode(element, size_limit).unwrap();
    let v2 = bincode::to_vec(element, size_limit).unwrap();
    assert_eq!(v1, v2);

    v1
}

fn proxy_decode<V>(slice: &[u8]) -> V
    where V: Encodable + Decodable + serde::Serialize + serde::Deserialize + PartialEq + Debug + 'static
{
    let e1 = bincode::decode(slice).unwrap();
    let e2 = bincode::from_slice(slice).unwrap();

    assert_eq!(e1, e2);

    e1
}

fn proxy_encoded_size<V>(element: &V) -> u64
    where V: Encodable+Decodable+serde::Serialize+serde::Deserialize+PartialEq+Debug+'static
{
    let ser_size = bincode::encoded_size(element);
    let serde_size = bincode::serialized_size(element);
    assert_eq!(ser_size, serde_size);
    ser_size
}

fn the_same<V>(element: V)
    where V: Encodable+Decodable+serde::Serialize+serde::Deserialize+PartialEq+Debug+'static
{
    // Make sure that the bahavior isize correct when wrapping with a RefBox.
    fn ref_box_correct<V>(v: &V) -> bool
        where V: Encodable + Decodable + PartialEq + Debug + 'static
    {
        let rf = RefBox::new(v);
        let encoded = encode(&rf, Infinite).unwrap();
        let decoded: RefBox<'static, V> = decode(&encoded[..]).unwrap();

        decoded.take().deref() == v
    }

    let size = proxy_encoded_size(&element);

    let encoded = proxy_encode(&element, Infinite);
    let decoded = proxy_decode(&encoded[..]);

    assert_eq!(element, decoded);
    assert_eq!(size, encoded.len() as u64);
    assert!(ref_box_correct(&element));
}

#[test]
fn test_numbers() {
    // unsigned positive
    the_same(5u8);
    the_same(5u16);
    the_same(5u32);
    the_same(5u64);
    the_same(5usize);
    // signed positive
    the_same(5i8);
    the_same(5i16);
    the_same(5i32);
    the_same(5i64);
    the_same(5isize);
    // signed negative
    the_same(-5i8);
    the_same(-5i16);
    the_same(-5i32);
    the_same(-5i64);
    the_same(-5isize);
    // floating
    the_same(-100f32);
    the_same(0f32);
    the_same(5f32);
    the_same(-100f64);
    the_same(5f64);
}

#[test]
fn test_string() {
    the_same("".to_string());
    the_same("a".to_string());
}

#[test]
fn test_tuple() {
    the_same((1isize,));
    the_same((1isize,2isize,3isize));
    the_same((1isize,"foo".to_string(),()));
}

#[test]
fn test_basic_struct() {
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    struct Easy {
        x: isize,
        s: String,
        y: usize
    }
    the_same(Easy{x: -4, s: "foo".to_string(), y: 10});
}

#[test]
fn test_nested_struct() {
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    struct Easy {
        x: isize,
        s: String,
        y: usize
    }
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    struct Nest {
        f: Easy,
        b: usize,
        s: Easy
    }

    the_same(Nest {
        f: Easy {x: -1, s: "foo".to_string(), y: 20},
        b: 100,
        s: Easy {x: -100, s: "bar".to_string(), y: 20}
    });
}

#[test]
fn test_struct_newtype() {
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    struct NewtypeStr(usize);

    the_same(NewtypeStr(5));
}

#[test]
fn test_struct_tuple() {
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    struct TubStr(usize, String, f32);

    the_same(TubStr(5, "hello".to_string(), 3.2));
}

#[test]
fn test_option() {
    the_same(Some(5usize));
    the_same(Some("foo bar".to_string()));
    the_same(None::<usize>);
}

#[test]
fn test_enum() {
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, PartialEq, Debug)]
    enum TestEnum {
        NoArg,
        OneArg(usize),
        Args(usize, usize),
        AnotherNoArg,
        StructLike{x: usize, y: f32}
    }
    the_same(TestEnum::NoArg);
    the_same(TestEnum::OneArg(4));
    the_same(TestEnum::Args(4, 5));
    the_same(TestEnum::AnotherNoArg);
    the_same(TestEnum::StructLike{x: 4, y: 3.14159});
    the_same(vec![TestEnum::NoArg, TestEnum::OneArg(5), TestEnum::AnotherNoArg,
                  TestEnum::StructLike{x: 4, y:1.4}]);
}

#[test]
fn test_vec() {
    let v: Vec<u8> = vec![];
    the_same(v);
    the_same(vec![1u64]);
    the_same(vec![1u64,2,3,4,5,6]);
}

#[test]
fn test_map() {
    let mut m = HashMap::new();
    m.insert(4u64, "foo".to_string());
    m.insert(0u64, "bar".to_string());
    the_same(m);
}

#[test]
fn test_bool() {
    the_same(true);
    the_same(false);
}

#[test]
fn test_unicode() {
    the_same("å".to_string());
    the_same("aåååååååa".to_string());
}

#[test]
fn decoding_errors() {
    fn isize_invalid_encoding<T>(res: DecodingResult<T>) {
        match res {
            Ok(_) => panic!("Expecting error"),
            Err(DecodingError::IoError(_)) => panic!("Expecting InvalidEncoding"),
            Err(DecodingError::SizeLimit) => panic!("Expecting InvalidEncoding"),
            Err(DecodingError::InvalidEncoding(_)) => {},
        }
    }

    isize_invalid_encoding(decode::<bool>(&vec![0xA][..]));
    isize_invalid_encoding(decode::<String>(&vec![0, 0, 0, 0, 0, 0, 0, 1, 0xFF][..]));
    // Out-of-bounds variant
    #[derive(RustcEncodable, RustcDecodable, Serialize)]
    enum Test {
        One,
        Two,
    };
    isize_invalid_encoding(decode::<Test>(&vec![0, 0, 0, 5][..]));
    isize_invalid_encoding(decode::<Option<u8>>(&vec![5, 0][..]));
}

#[test]
fn deserializing_errors() {
    fn isize_invalid_deserialize<T: Debug>(res: DeserializeResult<T>) {
        match res {
            Err(DeserializeError::InvalidEncoding(_)) => {},
            Err(DeserializeError::SyntaxError) => {},
            _ => panic!("Expecting InvalidEncoding, got {:?}", res),
        }
    }

    isize_invalid_deserialize(bincode::from_slice::<bool>(&vec![0xA][..]));
    isize_invalid_deserialize(bincode::from_slice::<String>(&vec![0, 0, 0, 0, 0, 0, 0, 1, 0xFF][..]));
    // Out-of-bounds variant
    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, Debug)]
    enum Test {
        One,
        Two,
    };
    isize_invalid_deserialize(bincode::from_slice::<Test>(&vec![0, 0, 0, 5][..]));
    isize_invalid_deserialize(bincode::from_slice::<Option<u8>>(&vec![5, 0][..]));
}

#[test]
fn too_big_decode() {
    let encoded = vec![0,0,0,3];
    let decoded: Result<u32, _> = decode_from(&mut &encoded[..], Bounded(3));
    assert!(decoded.is_err());

    let encoded = vec![0,0,0,3];
    let decoded: Result<u32, _> = decode_from(&mut &encoded[..], Bounded(4));
    assert!(decoded.is_ok());
}

#[test]
fn too_big_deserialize() {
    let serialized = vec![0,0,0,3];
    let deserialized: Result<u32, _> = bincode::from_reader(&mut &serialized[..], Bounded(3));
    assert!(deserialized.is_err());

    let serialized = vec![0,0,0,3];
    let deserialized: Result<u32, _> = bincode::from_reader(&mut &serialized[..], Bounded(4));
    assert!(deserialized.is_ok());
}

#[test]
fn too_big_char_decode() {
    let encoded = vec![0x41];
    let decoded: Result<char, _> = decode_from(&mut &encoded[..], Bounded(1));
    assert!(decoded.is_ok());
    assert_eq!(decoded.unwrap(), 'A');
}

#[test]
fn too_big_char_deserialize() {
    let serialized = vec![0x41];
    let deserialized: Result<char, _> = bincode::from_reader(&mut &serialized[..], Bounded(1));
    assert!(deserialized.is_ok());
    assert_eq!(deserialized.unwrap(), 'A');
}

#[test]
fn too_big_encode() {
    assert!(encode(&0u32, Bounded(3)).is_err());
    assert!(encode(&0u32, Bounded(4)).is_ok());

    assert!(encode(&"abcde", Bounded(8 + 4)).is_err());
    assert!(encode(&"abcde", Bounded(8 + 5)).is_ok());
}

#[test]
fn too_big_serialize() {
    assert!(bincode::to_vec(&0u32, Bounded(3)).is_err());
    assert!(bincode::to_vec(&0u32, Bounded(4)).is_ok());

    assert!(bincode::to_vec(&"abcde", Bounded(8 + 4)).is_err());
    assert!(bincode::to_vec(&"abcde", Bounded(8 + 5)).is_ok());
}

#[test]
fn test_encoded_size() {
    assert!(encoded_size(&0u8) == 1);
    assert!(encoded_size(&0u16) == 2);
    assert!(encoded_size(&0u32) == 4);
    assert!(encoded_size(&0u64) == 8);

    // length isize stored as u64
    assert!(encoded_size(&"") == 8);
    assert!(encoded_size(&"a") == 8 + 1);

    assert!(encoded_size(&vec![0u32, 1u32, 2u32]) == 8 + 3 * (4))

}

#[test]
fn test_serialized_size() {
    assert!(bincode::serialized_size(&0u8) == 1);
    assert!(bincode::serialized_size(&0u16) == 2);
    assert!(bincode::serialized_size(&0u32) == 4);
    assert!(bincode::serialized_size(&0u64) == 8);

    // length isize stored as u64
    assert!(bincode::serialized_size(&"") == 8);
    assert!(bincode::serialized_size(&"a") == 8 + 1);

    assert!(bincode::serialized_size(&vec![0u32, 1u32, 2u32]) == 8 + 3 * (4))
}

#[test]
fn encode_box() {
    the_same(Box::new(5));
}

#[test]
fn test_refbox_encode() {
    let large_object = vec![1u32,2,3,4,5,6];
    let mut large_map = HashMap::new();
    large_map.insert(1, 2);


    #[derive(RustcEncodable, RustcDecodable, Debug)]
    enum Message<'a> {
        M1(RefBox<'a, Vec<u32>>),
        M2(RefBox<'a, HashMap<u32, u32>>)
    }

    // Test 1
    {
        let encoded = encode(&Message::M1(RefBox::new(&large_object)), Infinite).unwrap();
        let decoded: Message<'static> = decode(&encoded[..]).unwrap();

        match decoded {
            Message::M1(b) => assert!(b.take().deref() == &large_object),
            _ => assert!(false)
        }
    }

    // Test 2
    {
        let encoded = encode(&Message::M2(RefBox::new(&large_map)), Infinite).unwrap();
        let decoded: Message<'static> = decode(&encoded[..]).unwrap();

        match decoded {
            Message::M2(b) => assert!(b.take().deref() == &large_map),
            _ => assert!(false)
        }
    }
}

#[test]
fn test_refbox_serialize() {
    let large_object = vec![1u32,2,3,4,5,6];
    let mut large_map = HashMap::new();
    large_map.insert(1, 2);


    #[derive(RustcEncodable, RustcDecodable, Serialize, Deserialize, Debug)]
    enum Message<'a> {
        M1(RefBox<'a, Vec<u32>>),
        M2(RefBox<'a, HashMap<u32, u32>>)
    }

    // Test 1
    {
        let serialized = bincode::to_vec(&Message::M1(RefBox::new(&large_object)), Infinite).unwrap();
        let deserialized: Message<'static> = bincode::from_reader(&mut &serialized[..], Infinite).unwrap();

        match deserialized {
            Message::M1(b) => assert!(b.take().deref() == &large_object),
            _ => assert!(false)
        }
    }

    // Test 2
    {
        let serialized = bincode::to_vec(&Message::M2(RefBox::new(&large_map)), Infinite).unwrap();
        let deserialized: Message<'static> = bincode::from_reader(&mut &serialized[..], Infinite).unwrap();

        match deserialized {
            Message::M2(b) => assert!(b.take().deref() == &large_map),
            _ => assert!(false)
        }
    }
}

#[test]
fn test_strbox_encode() {
    let strx: &'static str = "hello world";
    let encoded = encode(&StrBox::new(strx), Infinite).unwrap();
    let decoded: StrBox<'static> = decode(&encoded[..]).unwrap();
    let stringx: String = decoded.take();
    assert!(strx == &stringx[..]);
}

#[test]
fn test_strbox_serialize() {
    let strx: &'static str = "hello world";
    let serialized = bincode::to_vec(&StrBox::new(strx), Infinite).unwrap();
    let deserialized: StrBox<'static> = bincode::from_reader(&mut &serialized[..], Infinite).unwrap();
    let stringx: String = deserialized.take();
    assert!(strx == &stringx[..]);
}

#[test]
fn test_slicebox_encode() {
    let slice = [1u32, 2, 3 ,4, 5];
    let encoded = encode(&SliceBox::new(&slice), Infinite).unwrap();
    let decoded: SliceBox<'static, u32> = decode(&encoded[..]).unwrap();
    {
        let sb: &[u32] = &decoded;
        assert!(slice == sb);
    }
    let vecx: Vec<u32> = decoded.take();
    assert!(slice == &vecx[..]);
}

#[test]
fn test_slicebox_serialize() {
    let slice = [1u32, 2, 3 ,4, 5];
    let serialized = bincode::to_vec(&SliceBox::new(&slice), Infinite).unwrap();
    let deserialized: SliceBox<'static, u32> = bincode::from_reader(&mut &serialized[..], Infinite).unwrap();
    {
        let sb: &[u32] = &deserialized;
        assert!(slice == sb);
    }
    let vecx: Vec<u32> = deserialized.take();
    assert!(slice == &vecx[..]);
}

#[test]
fn test_multi_strings_encode() {
    assert!(encode(&("foo", "bar", "baz"), Infinite).is_ok());
}

#[test]
fn test_multi_strings_serialize() {
    assert!(bincode::to_vec(&("foo", "bar", "baz"), Infinite).is_ok());
}
