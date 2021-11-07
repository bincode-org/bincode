use crate::generate::Generator;
use crate::parse::Fields;
use crate::prelude::Delimiter;
use crate::Result;

pub struct DeriveStruct {
    pub fields: Fields,
}

impl DeriveStruct {
    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        generator
            .impl_for("bincode::enc::Encode")
            .unwrap()
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encoder"])
            .with_self_arg(crate::generate::FnSelfArg::RefSelf)
            .with_arg("mut encoder", "E")
            .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                for field in fields.names() {
                    fn_body
                        .push_parsed(format!(
                            "bincode::enc::Encode::encode(&self.{}, &mut encoder)?;",
                            field.to_string()
                        ))
                        .unwrap();
                }
                fn_body.push_parsed("Ok(())").unwrap();
            })
            .unwrap();

        Ok(())
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode
        let DeriveStruct { fields } = self;

        // struct has no lifetimes, implement Decode

        generator
            .impl_for("bincode::Decode")
            .unwrap()
            .generate_fn("decode")
            .with_generic("D", ["bincode::de::Decoder"])
            .with_arg("mut decoder", "D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        // Fields
                        // {
                        //      a: bincode::Decode::decode(&mut decoder)?,
                        //      b: bincode::Decode::decode(&mut decoder)?,
                        //      ...
                        // }
                        for field in fields.names() {
                            struct_body
                                .push_parsed(format!(
                                    "{}: bincode::Decode::decode(&mut decoder)?,",
                                    field.to_string()
                                ))
                                .unwrap();
                        }
                    });
                });
            })
            .unwrap();

        Ok(())
    }

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode
        let DeriveStruct { fields } = self;

        generator
            .impl_for_with_de_lifetime("bincode::de::BorrowDecode<'__de>")
            .unwrap()
            .generate_fn("borrow_decode")
            .with_generic("D", ["bincode::de::BorrowDecoder<'__de>"])
            .with_arg("mut decoder", "D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
            .body(|fn_body| {
                // Ok(Self {
                fn_body.ident_str("Ok");
                fn_body.group(Delimiter::Parenthesis, |ok_group| {
                    ok_group.ident_str("Self");
                    ok_group.group(Delimiter::Brace, |struct_body| {
                        for field in fields.names() {
                            struct_body
                                .push_parsed(format!(
                                    "{}: bincode::de::BorrowDecode::borrow_decode(&mut decoder)?,",
                                    field.to_string()
                                ))
                                .unwrap();
                        }
                    });
                });
            })
            .unwrap();

        Ok(())
    }
}
