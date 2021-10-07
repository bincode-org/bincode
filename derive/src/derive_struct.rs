use crate::generate::Generator;
use crate::parse::Field;
use crate::prelude::Delimiter;
use crate::Result;

pub struct DeriveStruct {
    pub fields: Vec<Field>,
}

impl DeriveStruct {
    pub fn generate_encodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        let mut impl_for = generator.impl_for("bincode::enc::Encodeable");
        let mut fn_body = impl_for.generate_fn("encode", |fn_def| {
            fn_def
                .with_generic("E", ["bincode::enc::Encode"])
                .with_self_arg(crate::generate::FnSelfArg::RefSelf)
                .with_arg("mut encoder", "E")
                .with_return_type("Result<(), bincode::error::EncodeError>")
        });
        let fn_body = fn_body.stream();
        for (idx, field) in fields.iter().enumerate() {
            let field_name = field
                .ident
                .as_ref()
                .map(|idx| idx.to_string())
                .unwrap_or_else(|| idx.to_string());
            fn_body.push_parsed(format!(
                "bincode::enc::Encodeable::encode(&self.{}, &mut encoder)?;",
                field_name
            ));
        }
        fn_body.push_parsed("Ok(())");

        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        if generator.has_lifetimes() {
            // struct has a lifetime, implement BorrowDecodable

            let mut impl_for =
                generator.impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>");
            let mut fn_builder = impl_for.generate_fn("borrow_decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });
            let fn_body = fn_builder.stream();
            // Ok(Self {
            fn_body.ident_str("Ok");
            fn_body.group(Delimiter::Parenthesis, |ok_group| {
                ok_group.ident_str("Self");
                ok_group.group(Delimiter::Brace, |struct_body| {
                    for (idx, field) in fields.into_iter().enumerate() {
                        let field_name_or_number = field
                            .ident
                            .map(|i| i.to_string())
                            .unwrap_or_else(|| idx.to_string());
                        struct_body.push_parsed(format!(
                            "{}: bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,",
                            field_name_or_number
                        ));
                    }
                });
            });

            Ok(())
        } else {
            // struct has no lifetimes, implement Decodable

            let mut impl_for = generator.impl_for("bincode::de::Decodable");
            let mut fn_builder = impl_for.generate_fn("decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::Decode"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });

            let fn_body = fn_builder.stream();
            // Ok(Self {
            fn_body.ident_str("Ok");
            fn_body.group(Delimiter::Parenthesis, |ok_group| {
                ok_group.ident_str("Self");
                ok_group.group(Delimiter::Brace, |struct_body| {
                    for (idx, field) in fields.into_iter().enumerate() {
                        let field_name_or_number = field
                            .ident
                            .map(|i| i.to_string())
                            .unwrap_or_else(|| idx.to_string());
                        struct_body.push_parsed(format!(
                            "{}: bincode::de::Decodable::decode(&mut decoder)?,",
                            field_name_or_number
                        ));
                    }
                });
            });

            Ok(())
        }
    }
}
