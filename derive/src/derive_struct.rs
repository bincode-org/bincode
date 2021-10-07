use crate::generate::Generator;
use crate::parse::Field;
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
        for (idx, field) in fields.iter().enumerate() {
            let field_name = field
                .ident
                .as_ref()
                .map(|idx| idx.to_string())
                .unwrap_or_else(|| idx.to_string());
            fn_body.push(format!(
                "bincode::enc::Encodeable::encode(&self.{}, &mut encoder)?;",
                field_name
            ));
        }
        fn_body.push("Ok(())");

        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveStruct { fields } = self;

        if generator.has_lifetimes() {
            // struct has a lifetime, implement BorrowDecodable

            // impl #impl_generics bincode::de::BorrowDecodable<'__de> for #name #ty_generics #where_clause {
            //     fn borrow_decode<D: bincode::de::BorrowDecode<'__de>>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {

            let mut impl_for =
                generator.impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>");
            let mut fn_builder = impl_for.generate_fn("borrow_decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });
            let mut body = String::new();
            body += "Ok(Self {";
            for (idx, field) in fields.into_iter().enumerate() {
                let field_name_or_number = field
                    .ident
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| idx.to_string());

                body += &format!(
                    "{}: bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,",
                    field_name_or_number
                );
            }
            body += "})";
            fn_builder.push(body);
        } else {
            // struct has no lifetimes, implement Decodable

            // impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
            //     fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {

            let mut impl_for = generator.impl_for("bincode::de::Decodable");
            let mut fn_builder = impl_for.generate_fn("decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::Decode"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });
            let mut body = String::new();
            body += "Ok(Self {";
            for (idx, field) in fields.into_iter().enumerate() {
                let field_name_or_number = field
                    .ident
                    .map(|i| i.to_string())
                    .unwrap_or_else(|| idx.to_string());

                body += &format!(
                    "{}: bincode::de::Decodable::decode(&mut decoder)?,",
                    field_name_or_number
                );
            }
            body += "})";
            fn_builder.push(body);
        };

        Ok(())
    }
}
