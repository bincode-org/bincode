use crate::parse::{Field, GenericConstraints, Generics};
use crate::prelude::{Ident, TokenStream};
use crate::Result;

pub struct DeriveStruct {
    pub name: Ident,
    pub generics: Option<Generics>,
    pub generic_constraints: Option<GenericConstraints>,
    pub fields: Vec<Field>,
}

impl DeriveStruct {
    pub fn generate_encodable(self) -> Result<TokenStream> {
        let DeriveStruct {
            name,
            generics,
            generic_constraints,
            fields,
        } = self;

        let mut result = crate::generate::Generate::impl_for(
            "bincode::enc::Encodeable",
            &name,
            &generics,
            &generic_constraints,
        );
        {
            let mut fn_body = result.generate_fn("encode", |fn_def| {
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
        }

        Ok(result.build())
    }

    pub fn generate_decodable(self) -> Result<TokenStream> {
        let DeriveStruct {
            name,
            generics,
            generic_constraints,
            fields,
        } = self;

        let mut result;

        if generics.as_ref().map(|g| g.has_lifetime()).unwrap_or(false) {
            // struct has a lifetime, implement BorrowDecodable

            // impl #impl_generics bincode::de::BorrowDecodable<'__de> for #name #ty_generics #where_clause {
            //     fn borrow_decode<D: bincode::de::BorrowDecode<'__de>>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {

            result = crate::generate::Generate::impl_for_with_de_lifetime(
                "bincode::de::BorrowDecodable<'__de>",
                &name,
                &generics,
                &generic_constraints,
            );
            let mut fn_builder = result.generate_fn("borrow_decode", |builder| {
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
            fn_builder.finish();
        } else {
            // struct has no lifetimes, implement Decodable

            // impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
            //     fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {

            result = crate::generate::Generate::impl_for(
                "bincode::de::Decodable",
                &name,
                &generics,
                &generic_constraints,
            );
            let mut fn_builder = result.generate_fn("decode", |builder| {
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
            fn_builder.finish();
        };

        Ok(result.build())

        /*let DeriveStruct {
            name,
            generics,
            fields,
        } = self;

        let (mut impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        // check if the type has lifetimes
        let mut should_insert_lifetime = false;

        for param in &generics.params {
            if let GenericParam::Lifetime(_) = param {
                should_insert_lifetime = true;
                break;
            }
        }

        // if the type has lifetimes, insert '__de and bound it to the lifetimes
        let mut generics_with_decode_lifetime;
        if should_insert_lifetime {
            generics_with_decode_lifetime = generics.clone();
            let mut new_lifetime = LifetimeDef::new(Lifetime::new("'__de", Span::call_site()));

            for param in &generics.params {
                if let GenericParam::Lifetime(lt) = param {
                    new_lifetime.bounds.push(lt.lifetime.clone())
                }
            }
            generics_with_decode_lifetime
                .params
                .push(GenericParam::Lifetime(new_lifetime));

            impl_generics = generics_with_decode_lifetime.split_for_impl().0;
        }

        let fields = fields
            .into_iter()
            .map(|field| {
                if should_insert_lifetime {
                    quote! {
                        #field: bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,
                    }
                } else {
                    quote! {
                        #field: bincode::de::Decodable::decode(&mut decoder)?,
                    }
                }
            })
            .collect::<Vec<_>>();

        let result = if should_insert_lifetime {
            quote! {
                impl #impl_generics bincode::de::BorrowDecodable<'__de> for #name #ty_generics #where_clause {
                    fn borrow_decode<D: bincode::de::BorrowDecode<'__de>>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
                        Ok(#name {
                            #(#fields)*
                        })
                    }

                }
            }
        } else {
            quote! {
                impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
                    fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
                        Ok(#name {
                            #(#fields)*
                        })
                    }

                }
            }
        };

        Ok(result.into())*/
    }
}
