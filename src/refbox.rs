use std::boxed::Box;
use std::ops::Deref;

use rustc_serialize::{Encodable, Encoder};
use rustc_serialize::{Decodable, Decoder};

pub struct RefBox<'a, T: 'a> {
    inner:  RefBoxInner<'a, T>
}

#[derive(Debug, Hash)]
enum RefBoxInner<'a, T: 'a> {
    Ref(&'a T),
    Box(Box<T>)
}

impl <'a, T> RefBox<'a, T> {
    pub fn new(v: &'a T) -> RefBox<'a, T> {
        RefBox {
            inner: RefBoxInner::Ref(v)
        }
    }
}

impl <T> RefBox<'static, T>  {
    pub fn take(self) -> Box<T> {
        match self.inner {
            RefBoxInner::Box(b) => b,
            _ => unreachable!()
        }
    }
    pub fn try_take(self) -> Result<Box<T>, RefBox<'static, T>> {
        match self.inner {
            RefBoxInner::Box(b) => Ok(b),
            o => Err(RefBox{ inner: o})
        }
    }
}
impl <'a, T: Encodable> Encodable for RefBox<'a, T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        self.inner.encode(s)
    }
}
impl <T: Decodable> Decodable for RefBox<'static, T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<RefBox<'static, T>, D::Error> {
        let inner = try!(Decodable::decode(d));
        Ok(RefBox{inner: inner})
    }
}

impl <'a, T: Encodable> Encodable for RefBoxInner<'a, T> {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("RefBox", |s| {
            s.emit_enum_variant("Box", 1, 1, |s| {
                s.emit_enum_variant_arg(0, |s| {
                    match self {
                        &RefBoxInner::Ref(ref r) => r.encode(s),
                        &RefBoxInner::Box(ref b) => b.encode(s)
                    }
                })
            })
        })
    }
}

impl <T: Decodable> Decodable for RefBoxInner<'static, T> {
    fn decode<D: Decoder>(d: &mut D) -> Result<RefBoxInner<'static, T>, D::Error> {
        d.read_enum("RefBox", |d| {
            d.read_enum_variant(&["Ref", "Box"], |d, i| {
                assert!(i == 1);
                d.read_enum_variant_arg(0, |d| {
                    let decoded = try!(Decodable::decode(d));
                    Ok(RefBoxInner::Box(Box::new(decoded)))
                })
            })
        })
    }
}

impl <'a, T> Deref for RefBox<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        match &self.inner {
            &RefBoxInner::Ref(ref t) => t,
            &RefBoxInner::Box(ref b) => b.deref()
        }
    }
}

