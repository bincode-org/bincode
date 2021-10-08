use crate::generate::Generator;
use crate::parse::Fields;
use crate::prelude::Delimiter;
use crate::Result;

pub struct DeriveStruct {
    pub fields: Fields,
}

impl DeriveStruct {
    pub fn generate_encodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        let mut impl_for = generator.impl_for("bincode::enc::Encodeable");
        impl_for
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encode"])
            .with_self_arg(crate::generate::FnSelfArg::RefSelf)
            .with_arg("mut encoder", "E")
            .with_return_type("Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                for field in fields.names() {
                    fn_body.push_parsed(format!(
                        "bincode::enc::Encodeable::encode(&self.{}, &mut encoder)?;",
                        field.to_string()
                    ));
                }
                fn_body.push_parsed("Ok(())");
            });

        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        if generator.has_lifetimes() {
            // struct has a lifetime, implement BorrowDecodable

            generator
                .impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>")
                .generate_fn("borrow_decode")
                .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_body| {
                    // Ok(Self {
                    fn_body.ident_str("Ok");
                    fn_body.group(Delimiter::Parenthesis, |ok_group| {
                        ok_group.ident_str("Self");
                        ok_group.group(Delimiter::Brace, |struct_body| {
                            for field in fields.names() {
                                struct_body.push_parsed(format!(
                            "{}: bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,",
                            field.to_string()
                        ));
                            }
                        });
                    });
                });

            Ok(())
        } else {
            // struct has no lifetimes, implement Decodable

            let mut impl_for = generator.impl_for("bincode::de::Decodable");
            impl_for
                .generate_fn("decode")
                .with_generic("D", ["bincode::de::Decode"])
                .with_arg("mut decoder", "D")
                .with_return_type("Result<Self, bincode::error::DecodeError>")
                .body(|fn_body| {
                    // Ok(Self {
                    fn_body.ident_str("Ok");
                    fn_body.group(Delimiter::Parenthesis, |ok_group| {
                        ok_group.ident_str("Self");
                        ok_group.group(Delimiter::Brace, |struct_body| {
                            // Fields
                            // {
                            //      a: bincode::de::Decodable::decode(&mut decoder)?,
                            //      b: bincode::de::Decodable::decode(&mut decoder)?,
                            //      ...
                            // }
                            for field in fields.names() {
                                struct_body.push_parsed(format!(
                                    "{}: bincode::de::Decodable::decode(&mut decoder)?,",
                                    field.to_string()
                                ));
                            }
                        });
                    });
                });

            Ok(())
        }
    }
}
