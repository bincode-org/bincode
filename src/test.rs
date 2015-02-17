extern crate "rustc-serialize" as serialize;

use std::fmt::Debug;
use std::collections::HashMap;
use std::ops::Deref;

use rustc_serialize::{
    Encoder,
    Decoder,
    Encodable,
    Decodable
};

use super::{
    encode,
    decode,
    decode_from,
    encoded_size,
    DecodingError,
    DecodingResult,
    RefBox,
};

use super::SizeLimit::{Infinite, Bounded};

fn the_same<V>(element: V)
where V: Encodable + Decodable + PartialEq + Debug + 'static {

    // Make sure that the bahavior is correct when wrapping with a RefBox.
    fn ref_box_correct<V>(v: &V) -> bool
    where V: Encodable + Decodable + PartialEq + Debug + 'static {
        let rf = RefBox::new(v);

        let encoded = encode(&rf, Infinite).unwrap();
        let decoded: RefBox<'static, V> = decode(&encoded[]).unwrap();

        decoded.take().deref() == v
    }

    let size = encoded_size(&element);
    let encoded = encode(&element, Infinite).unwrap();
    let decoded = decode(&encoded[]).unwrap();
    assert!(element == decoded);
    assert!(size == encoded.len() as u64);
    assert!(ref_box_correct(&element))
}

#[test]
fn test_numbers() {
    // unsigned positive
    the_same(5u8);
    the_same(5u16);
    the_same(5u32);
    the_same(5u64);
    // signed positive
    the_same(5i8);
    the_same(5i16);
    the_same(5i32);
    the_same(5i64);
    // signed negative
    the_same(-5i8);
    the_same(-5i16);
    the_same(-5i32);
    the_same(-5i64);
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
    the_same((1is,));
    the_same((1is,2is,3is));
    the_same((1is,"foo".to_string(),()));
}

#[test]
fn test_basic_struct() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct Easy {
        x: isize,
        s: String,
        y: usize
    }
    the_same(Easy{x: -4, s: "foo".to_string(), y: 10});
}

#[test]
fn test_nested_struct() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct Easy {
        x: isize,
        s: String,
        y: usize
    }
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
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
fn test_struct_tuple() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct TubStr(usize, String, f32);

    the_same(TubStr(5, "hello".to_string(), 3.2));
}

#[test]
fn option() {
    the_same(Some(5us));
    the_same(Some("foo bar".to_string()));
    the_same(None::<usize>);
}

#[test]
fn enm() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    enum TestEnum {
        NoArg,
        OneArg(usize),
        AnotherNoArg
    }
    the_same(TestEnum::NoArg);
    the_same(TestEnum::OneArg(4));
    the_same(TestEnum::AnotherNoArg);
}


#[test]
fn struct_enum() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    enum TestEnum {
        NoArg,
        OneArg(usize),
        AnotherNoArg,
        StructLike{x: usize, y: f32}
    }
    the_same(TestEnum::NoArg);
    the_same(TestEnum::OneArg(4));
    the_same(TestEnum::AnotherNoArg);
    the_same(TestEnum::StructLike{x: 4, y: 3.14159});
    the_same(vec![TestEnum::NoArg, TestEnum::OneArg(5), TestEnum::AnotherNoArg,
                  TestEnum::StructLike{x: 4, y:1.4}]);
}

#[test]
fn many() {
    let v: Vec<u8> = vec![];
    the_same(v);
    the_same(vec![1u64]);
    the_same(vec![1u64,2,3,4,5,6]);
}

#[test]
fn map(){
    let mut m = HashMap::new();
    m.insert(4u64, "foo".to_string());
    m.insert(0u64, "bar".to_string());
    the_same(m);
}

#[test]
fn boole(){
    the_same(true);
    the_same(false);
}

#[test]
fn unicode() {
    the_same("å".to_string());
    the_same("aåååååååa".to_string());
}


#[test]
fn decoding_errors() {
    fn is_invalid_encoding<T>(res: DecodingResult<T>) {
        match res {
            Ok(_) => panic!("Expecting error"),
            Err(DecodingError::IoError(_)) => panic!("Expecting InvalidEncoding"),
            Err(DecodingError::SizeLimit) => panic!("Expecting InvalidEncoding"),
            Err(DecodingError::InvalidEncoding(_)) => {},
        }
    }

    is_invalid_encoding(decode::<bool>(vec![0xA].as_slice()));
    is_invalid_encoding(decode::<String>(vec![0, 0, 0, 0, 0, 0, 0, 1, 0xFF].as_slice()));
    // Out-of-bounds variant
    #[derive(RustcEncodable, RustcDecodable)]
    enum Test {
        One,
        Two,
    };
    is_invalid_encoding(decode::<Test>(vec![0, 0, 0, 5].as_slice()));
    is_invalid_encoding(decode::<Option<u8>>(vec![5, 0].as_slice()));
}

#[test]
fn too_big_decode() {
    let encoded = vec![0,0,0,3];
    let mut encoded_ref = encoded.as_slice();
    let decoded: Result<u32, _> = decode_from(&mut encoded_ref, Bounded(3));
    assert!(decoded.is_err());

    let encoded = vec![0,0,0,3];
    let mut encoded_ref = encoded.as_slice();
    let decoded: Result<u32, _> = decode_from(&mut encoded_ref, Bounded(4));
    assert!(decoded.is_ok());
}

#[test]
fn too_big_char_decode() {
    let encoded = vec![0x41];
    let mut encoded_ref = encoded.as_slice();
    let decoded: Result<char, _> = decode_from(&mut encoded_ref, Bounded(1));
    assert_eq!(decoded, Ok('A'));
}

#[test]
fn too_big_encode() {
    assert!(encode(&0u32, Bounded(3)).is_err());
    assert!(encode(&0u32, Bounded(4)).is_ok());

    assert!(encode(&"abcde", Bounded(8 + 4)).is_err());
    assert!(encode(&"abcde", Bounded(8 + 5)).is_ok());
}

#[test]
fn test_encoded_size() {
    assert!(encoded_size(&0u8) == 1);
    assert!(encoded_size(&0u16) == 2);
    assert!(encoded_size(&0u32) == 4);
    assert!(encoded_size(&0u64) == 8);

    // length is stored as u64
    assert!(encoded_size(&"") == 8);
    assert!(encoded_size(&"a") == 8 + 1);

    assert!(encoded_size(&vec![0u32, 1u32, 2u32]) == 8 + 3 * (4))
}

#[test]
fn encode_box() {
    the_same(Box::new(5));
}

#[test]
fn test_refbox() {
    let large_object = vec![1u32,2,3,4,5,6];
    let mut large_map = HashMap::new();
    large_map.insert(1, 2);


    #[derive(RustcEncodable, RustcDecodable)]
    enum Message<'a> {
        M1(RefBox<'a, Vec<u32>>),
        M2(RefBox<'a, HashMap<u32, u32>>)
    }

    // Test 1
    {
        let encoded = encode(&Message::M1(RefBox::new(&large_object)), Infinite).unwrap();
        let decoded: Message<'static> = decode(&encoded[]).unwrap();

        match decoded {
            Message::M1(b) => assert!(b.take().deref() == &large_object),
            _ => assert!(false)
        }
    }

    // Test 2
    {
        let encoded = encode(&Message::M2(RefBox::new(&large_map)), Infinite).unwrap();
        let decoded: Message<'static> = decode(&encoded[]).unwrap();

        match decoded {
            Message::M2(b) => assert!(b.take().deref() == &large_map),
            _ => assert!(false)
        }
    }
}
