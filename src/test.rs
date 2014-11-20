use std::io::MemWriter;
use std::fmt::Show;
use std::io::MemReader;
use std::io::IoError;
use std::collections::HashMap;

use serialize::{
    Encoder,
    Decoder,
    Encodable,
    Decodable
};

use super::EncoderWriter;
use super::DecoderReader;
use super::encode;
use super::decode;

fn the_same<'a,
            V: Encodable<EncoderWriter<'a, MemWriter>, IoError>  +
               Decodable<DecoderReader<'a, MemReader>, IoError> +
               PartialEq + Show>(element: V) {
    assert!(element == decode(encode(&element).unwrap()).unwrap());
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
    the_same((1i,));
    the_same((1i,2i,3i));
    the_same((1i,"foo".to_string(),()));
}

#[test]
fn test_basic_struct() {
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    struct Easy {
        x: int,
        s: String,
        y: uint
    }
    the_same(Easy{x: -4, s: "foo".to_string(), y: 10});
}

#[test]
fn test_nested_struct() {
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    struct Easy {
        x: int,
        s: String,
        y: uint
    }
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    struct Nest {
        f: Easy,
        b: uint,
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
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    struct TubStr(uint, String, f32);

    the_same(TubStr(5, "hello".to_string(), 3.2));
}

#[test]
fn option() {
    the_same(Some(5u));
    the_same(Some("foo bar".to_string()));
    the_same(None::<uint>);
}

#[test]
fn enm() {
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    enum TestEnum {
        NoArg,
        OneArg(uint),
        AnotherNoArg
    }
    the_same(TestEnum::NoArg);
    the_same(TestEnum::OneArg(4));
    the_same(TestEnum::AnotherNoArg);
}


#[test]
fn struct_enum() {
    #[deriving(Encodable, Decodable, PartialEq, Show)]
    enum TestEnum {
        NoArg,
        OneArg(uint),
        AnotherNoArg,
        StructLike{x: uint, y: f32}
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
    the_same(vec![1u]);
    the_same(vec![1u,2,3,4,5,6]);
}

#[test]
fn map(){
    let mut m = HashMap::new();
    m.insert(4u, "foo".to_string());
    m.insert(0u, "bar".to_string());
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
