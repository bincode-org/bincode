use crate::parse::{Field, GenericConstraints, Generics};
use crate::prelude::{Delimiter, Group, Ident, TokenStream, TokenTree};
use crate::Result;
use std::str::FromStr;

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

        let mut result = TokenStream::new();
        result.extend([TokenStream::from_str("impl").unwrap()]);
        if let Some(generics) = &generics {
            result.extend([generics.impl_generics()]);
        }
        result.extend([
            TokenStream::from_str("bincode::enc::Encodeable for").unwrap(),
            TokenTree::Ident(name).into(),
        ]);
        if let Some(generics) = &generics {
            result.extend([generics.type_generics()]);
        }
        if let Some(generic_constraints) = &generic_constraints {
            result.extend([generic_constraints.where_clause()]);
        }
        result.extend([
            TokenTree::Group(Group::new(Delimiter::Brace, {
                let mut fn_def = TokenStream::from_str("fn encode<E: bincode::enc::Encode>(&self, mut encoder: E) -> Result<(), bincode::error::EncodeError>").unwrap();
                let body = TokenTree::Group(Group::new(Delimiter::Brace, {
                    let mut stream = TokenStream::new();
                    for field in fields {
                        stream.extend([TokenStream::from_str(&format!("bincode::enc::Encodeable::encode(&self.{}, &mut encoder)?;", field.ident.unwrap())).unwrap()]);
                    }
                    stream.extend([TokenStream::from_str("Ok(())").unwrap()]);
                    stream
                }));
                fn_def.extend([body]);
                fn_def
            })),
        ]);

        Ok(result)
    }

    /*
    pub fn generate_decodable(self) -> Result<TokenStream> {
        let DeriveStruct {
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

        Ok(result.into())
    } */
}
