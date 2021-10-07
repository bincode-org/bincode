use crate::generate::{FnSelfArg, Generator};
use crate::parse::EnumVariant;
use crate::prelude::*;
use crate::utils::{ident, lit_u32, lit_usize, punct};
use crate::Result;
use std::str::FromStr;

pub struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    pub fn generate_encodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        let mut impl_for = generator.impl_for("bincode::enc::Encodeable");
        let mut fn_body = impl_for.generate_fn("encode", |builder| {
            builder
                .with_generic("E", ["bincode::enc::Encode"])
                .with_self_arg(FnSelfArg::RefSelf)
                .with_arg("mut encoder", "E")
                .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
        });

        fn_body.push_str("match self");
        fn_body.push_group(Delimiter::Brace, {
            let mut match_body = Vec::new();
            for (variant_index, variant) in variants.into_iter().enumerate() {
                // Self::Variant
                match_body.push(ident("Self"));
                match_body.push(punct(':'));
                match_body.push(punct(':'));
                match_body.push(TokenTree::Ident(variant.name.clone()));

                // if we have any fields, declare them here
                if let Some(fields) = variant.fields.as_ref() {
                    let delimiter = if variant.is_struct_variant() {
                        Delimiter::Brace
                    } else if variant.is_tuple_variant() {
                        Delimiter::Parenthesis
                    } else {
                        unreachable!()
                    };

                    // BlockedTODO: https://github.com/rust-lang/rust/issues/79524
                    // Use this code once intersperse is stabilized
                    // let field_body = fields.iter().enumerate().map(|(idx, field)|field.name_or_idx(idx)).intersperse(punct(',')).collect();

                    let field_body =
                        fields
                            .iter()
                            .enumerate()
                            .fold(Vec::new(), |mut target, (idx, field)| {
                                if !target.is_empty() {
                                    target.push(punct(','));
                                }
                                target.push(field.name_or_idx(idx).into_token_tree());
                                target
                            });
                    let mut stream = TokenStream::new();
                    stream.extend(field_body);

                    match_body.push(TokenTree::Group(Group::new(delimiter, stream)));
                }

                match_body.extend([
                    // Arrow
                    punct('='),
                    punct('>'),
                    // match body
                    TokenTree::Group(Group::new(Delimiter::Brace, {
                        let mut body = Vec::<TokenTree>::new();
                        // Encode the variant index
                        body.extend(
                            TokenStream::from_str(&format!(
                                "encoder.encode_u32({})?;",
                                variant_index
                            ))
                            .unwrap(),
                        );

                        if let Some(fields) = variant.fields.as_ref() {
                            // If we have any fields, encode them all one by one
                            for (idx, field) in fields.iter().enumerate() {
                                let line = format!(
                                    "bincode::enc::Encodeable::encode({}, &mut encoder)?;",
                                    field.name_or_idx(idx)
                                );
                                body.extend(TokenStream::from_str(&line).unwrap());
                            }
                        }

                        let mut stream = TokenStream::new();
                        stream.extend(body);
                        stream
                    })),
                    punct(','),
                ]);
            }
            match_body
        });
        fn_body.push_str("Ok(())");
        Ok(())
    }

    pub fn generate_decodable(self, generator: &mut Generator) -> Result<()> {
        let DeriveEnum { variants } = self;

        if generator.has_lifetimes() {
            // enum has a lifetime, implement BorrowDecodable

            let mut impl_for =
                generator.impl_for_with_de_lifetime("bincode::de::BorrowDecodable<'__de>");
            let mut fn_builder = impl_for.generate_fn("borrow_decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::BorrowDecode<'__de>"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });

            fn_builder
                .push_str("let variant_index = bincode::de::Decode::decode_u32(&mut decoder)?;");
            fn_builder.push_str("match variant_index");
            fn_builder.push_group(Delimiter::Brace, {
                let mut variant_case = Vec::new();
                for (idx, variant) in variants.iter().enumerate() {
                    // idx => Ok(..)
                    variant_case.extend([
                        lit_u32(idx as u32),
                        punct('='),
                        punct('>'),
                        ident("Ok"),
                        TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                            // Self::Variant
                            // Self::Variant { 0: ..., 1: ... 2: ... },
                            // Self::Variant { a: ..., b: ... c: ... },
                            let mut variant_case_body = vec![
                                ident("Self"),
                                punct(':'),
                                punct(':'),
                                TokenTree::Ident(variant.name.clone())
                            ];

                            if let Some(fields) = variant.fields.as_ref() {
                                variant_case_body.push(TokenTree::Group(Group::new(Delimiter::Brace, {
                                    let mut variant_body = Vec::<TokenTree>::new();
                                    for (idx, field) in fields.iter().enumerate() {
                                        variant_body.push(if let Some(ident) = field.ident.clone() {
                                            TokenTree::Ident(ident)
                                        } else {
                                            lit_usize(idx)
                                        });
                                        variant_body.push(punct(':'));
                                        variant_body.extend(TokenStream::from_str("bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,").unwrap());
                                    }
                                    let mut stream = TokenStream::new();
                                    stream.extend(variant_body);
                                    stream
                                })));
                            }

                            let mut stream = TokenStream::new();
                            stream.extend(variant_case_body);
                            stream
                        })),
                        punct(',')
                    ]);
                }

                // invalid idx
                let default_case = format!(
                    "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                    variants.len() - 1
                );
                variant_case.extend(TokenStream::from_str(&default_case).unwrap());

                let mut stream = TokenStream::new();
                stream.extend(variant_case);
                stream
            });
        } else {
            // enum has no lifetimes, implement Decodable

            let mut impl_for = generator.impl_for("bincode::de::Decodable");
            let mut fn_builder = impl_for.generate_fn("decode", |builder| {
                builder
                    .with_generic("D", ["bincode::de::Decode"])
                    .with_arg("mut decoder", "D")
                    .with_return_type("Result<Self, bincode::error::DecodeError>")
            });

            fn_builder
                .push_str("let variant_index = bincode::de::Decode::decode_u32(&mut decoder)?;");
            fn_builder.push_str("match variant_index");
            fn_builder.push_group(Delimiter::Brace, {
                let mut variant_case = Vec::new();
                for (idx, variant) in variants.iter().enumerate() {
                    // idx => Ok(..)
                    variant_case.extend([
                        lit_u32(idx as u32),
                        punct('='),
                        punct('>'),
                        ident("Ok"),
                        TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                            // Self::Variant
                            // Self::Variant { 0: ..., 1: ... 2: ... },
                            // Self::Variant { a: ..., b: ... c: ... },
                            let mut variant_case_body = vec![
                                ident("Self"),
                                punct(':'),
                                punct(':'),
                                TokenTree::Ident(variant.name.clone())
                            ];

                            if let Some(fields) = variant.fields.as_ref() {
                                variant_case_body.push(TokenTree::Group(Group::new(Delimiter::Brace, {
                                    let mut variant_body = Vec::<TokenTree>::new();
                                    for (idx, field) in fields.iter().enumerate() {
                                        variant_body.push(if let Some(ident) = field.ident.clone() {
                                            TokenTree::Ident(ident)
                                        } else {
                                            lit_usize(idx)
                                        });

                                        variant_body.push(punct(':'));
                                        variant_body.extend(TokenStream::from_str("bincode::de::Decodable::decode(&mut decoder)?,").unwrap());
                                    }
                                    let mut stream = TokenStream::new();
                                    stream.extend(variant_body);
                                    stream
                                })));
                            }

                            let mut stream = TokenStream::new();
                            stream.extend(variant_case_body);
                            stream
                        })),
                        punct(',')
                    ]);
                }

                // invalid idx
                let default_case = format!(
                    "variant => return Err(bincode::error::DecodeError::UnexpectedVariant {{ min: 0, max: {}, found: variant }})",
                    variants.len() - 1
                );
                variant_case.extend(TokenStream::from_str(&default_case).unwrap());

                let mut stream = TokenStream::new();
                stream.extend(variant_case);
                stream
            });
        }

        Ok(())
    }
    //     let max_variant = (variants.len() - 1) as u32;
    //     let match_arms = variants.iter().enumerate().map(|(index, variant)| {
    //         let index = index as u32;
    //         let decode_statements = field_names_to_decodable(
    //             &fields_to_constructable_names(&variant.fields),
    //             should_insert_lifetime,
    //         );
    //         let variant_name = variant.ident.clone();
    //         quote! {
    //             #index => {
    //                 #name :: #variant_name {
    //                     #(#decode_statements)*
    //                 }
    //             }
    //         }
    //     });
    //     let result = if should_insert_lifetime {
    //         quote! {
    //             impl #impl_generics bincode::de::BorrowDecodable<'__de> for #name #ty_generics #where_clause {
    //                 fn borrow_decode<D: bincode::de::BorrowDecode<'__de>>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
    //                     let i = decoder.decode_u32()?;
    //                     Ok(match i {
    //                         #(#match_arms)*
    //                         variant => return Err(bincode::error::DecodeError::UnexpectedVariant{
    //                             min: 0,
    //                             max: #max_variant,
    //                             found: variant,
    //                         })
    //                     })
    //                 }

    //             }
    //         }
    //     } else {
    //         quote! {
    //             impl #impl_generics bincode::de::Decodable for #name #ty_generics #where_clause {
    //                 fn decode<D: bincode::de::Decode>(mut decoder: D) -> Result<Self, bincode::error::DecodeError> {
    //                     let i = decoder.decode_u32()?;
    //                     Ok(match i {
    //                         #(#match_arms)*
    //                         variant => return Err(bincode::error::DecodeError::UnexpectedVariant{
    //                             min: 0,
    //                             max: #max_variant,
    //                             found: variant,
    //                         })
    //                     })
    //                 }

    //             }
    //         }
    //     };

    //     Ok(result.into())
    // }
}

// fn fields_to_match_arm(fields: &Fields) -> TokenStream2 {
//     match fields {
//         syn::Fields::Named(fields) => {
//             let fields: Vec<_> = fields
//                 .named
//                 .iter()
//                 .map(|f| f.ident.clone().unwrap().to_token_stream())
//                 .collect();
//             quote! {
//                 {#(#fields),*}
//             }
//         }
//         syn::Fields::Unnamed(fields) => {
//             let fields: Vec<_> = fields
//                 .unnamed
//                 .iter()
//                 .enumerate()
//                 .map(|(i, f)| Ident::new(&format!("_{}", i), f.span()))
//                 .collect();
//             quote! {
//                 (#(#fields),*)
//             }
//         }
//         syn::Fields::Unit => quote! {},
//     }
// }

// fn fields_to_names(fields: &Fields) -> Vec<TokenStream2> {
//     match fields {
//         syn::Fields::Named(fields) => fields
//             .named
//             .iter()
//             .map(|f| f.ident.clone().unwrap().to_token_stream())
//             .collect(),
//         syn::Fields::Unnamed(fields) => fields
//             .unnamed
//             .iter()
//             .enumerate()
//             .map(|(i, f)| Ident::new(&format!("_{}", i), f.span()).to_token_stream())
//             .collect(),
//         syn::Fields::Unit => Vec::new(),
//     }
// }

// fn field_names_to_encodable(names: &[TokenStream2]) -> Vec<TokenStream2> {
//     names
//         .iter()
//         .map(|field| {
//             quote! {
//                 bincode::enc::Encodeable::encode(#field, &mut encoder)?;
//             }
//         })
//         .collect::<Vec<_>>()
// }

// fn fields_to_constructable_names(fields: &Fields) -> Vec<TokenStream2> {
//     match fields {
//         syn::Fields::Named(fields) => fields
//             .named
//             .iter()
//             .map(|f| f.ident.clone().unwrap().to_token_stream())
//             .collect(),
//         syn::Fields::Unnamed(fields) => fields
//             .unnamed
//             .iter()
//             .enumerate()
//             .map(|(i, _)| Index::from(i).to_token_stream())
//             .collect(),
//         syn::Fields::Unit => Vec::new(),
//     }
// }

// fn field_names_to_decodable(names: &[TokenStream2], borrowed: bool) -> Vec<TokenStream2> {
//     names
//         .iter()
//         .map(|field| {
//             if borrowed {
//                 quote! {
//                     #field: bincode::de::BorrowDecodable::borrow_decode(&mut decoder)?,
//                 }
//             } else {
//                 quote! {
//                     #field: bincode::de::Decodable::decode(&mut decoder)?,
//                 }
//             }
//         })
//         .collect::<Vec<_>>()
// }
