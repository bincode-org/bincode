use crate::generate::{FnSelfArg, Generator, StreamBuilder};
use crate::parse::{EnumVariant, Fields};
use crate::prelude::*;
use crate::Result;

const TUPLE_FIELD_PREFIX: &str = "field_";

pub struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
}

impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            last_val: None,
            variants: &self.variants,
        }
    }

    pub fn generate_encode(self, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for("bincode::enc::Encode")
            .unwrap()
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encoder"])
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("mut encoder", "E")
            .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                fn_body.ident_str("match");
                fn_body.ident_str("self");
                fn_body.group(Delimiter::Brace, |match_body| {
                    for (variant_index, variant) in self.iter_fields() {
                        // Self::Variant
                        match_body.ident_str("Self");
                        match_body.puncts("::");
                        match_body.ident(variant.name.clone());

                        // if we have any fields, declare them here
                        // Self::Variant { a, b, c }
                        if let Some(delimiter) = variant.fields.delimiter() {
                            match_body.group(delimiter, |field_body| {
                                for (idx, field_name) in
                                    variant.fields.names().into_iter().enumerate()
                                {
                                    if idx != 0 {
                                        field_body.punct(',');
                                    }
                                    field_body.push(
                                        field_name.to_token_tree_with_prefix(TUPLE_FIELD_PREFIX),
                                    );
                                }
                            });
                        }

                        // Arrow
                        // Self::Variant { a, b, c } =>
                        match_body.puncts("=>");

                        // Body of this variant
                        // Note that the fields are available as locals because of the match destructuring above
                        // {
                        //      encoder.encode_u32(n)?;
                        //      bincode::enc::Encode::encode(a, &mut encoder)?;
                        //      bincode::enc::Encode::encode(b, &mut encoder)?;
                        //      bincode::enc::Encode::encode(c, &mut encoder)?;
                        // }
                        match_body.group(Delimiter::Brace, |body| {
                            // variant index
                            body.push_parsed("<u32 as bincode::enc::Encode>::encode")
                                .unwrap();
                            body.group(Delimiter::Parenthesis, |args| {
                                args.punct('&');
                                args.group(Delimiter::Parenthesis, |num| num.extend(variant_index));
                                args.punct(',');
                                args.push_parsed("&mut encoder").unwrap();
                            });
                            body.punct('?');
                            body.punct(';');
                            // If we have any fields, encode them all one by one
                            for field_name in variant.fields.names() {
                                body.push_parsed(format!(
                                    "bincode::enc::Encode::encode({}, &mut encoder)?;",
                                    field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                ))
                                .unwrap();
                            }
                        });
                        match_body.punct(',');
                    }
                });
                fn_body.push_parsed("Ok(())").unwrap();
            })
            .unwrap();
        Ok(())
    }

    /// Build the catch-all case for an int-to-enum decode implementation
    fn invalid_variant_case(&self, enum_name: &str, result: &mut StreamBuilder) {
        // we'll be generating:
        // variant => Err(
        //    bincode::error::DecodeError::UnexpectedVariant {
        //        found: variant,
        //        type_name: <enum_name>
        //        allowed: ...,
        //    }
        // )
        //
        // Where allowed is either:
        // - bincode::error::AllowedEnumVariants::Range { min: 0, max: <max> }
        //   if we have no fixed value variants
        // - bincode::error::AllowedEnumVariants::Allowed(&[<variant1>, <variant2>, ...])
        //   if we have fixed value variants
        result.ident_str("variant");
        result.puncts("=>");
        result.ident_str("Err");
        result.group(Delimiter::Parenthesis, |err_inner| {
            err_inner
                .push_parsed("bincode::error::DecodeError::UnexpectedVariant")
                .unwrap();
            err_inner.group(Delimiter::Brace, |variant_inner| {
                variant_inner.ident_str("found");
                variant_inner.punct(':');
                variant_inner.ident_str("variant");
                variant_inner.punct(',');

                variant_inner.ident_str("type_name");
                variant_inner.punct(':');
                variant_inner.lit_str(enum_name);
                variant_inner.punct(',');

                variant_inner.ident_str("allowed");
                variant_inner.punct(':');

                if self.variants.iter().any(|i| i.has_fixed_value()) {
                    // we have fixed values, implement AllowedEnumVariants::Allowed
                    variant_inner
                        .push_parsed("bincode::error::AllowedEnumVariants::Allowed")
                        .unwrap();
                    variant_inner.group(Delimiter::Parenthesis, |allowed_inner| {
                        allowed_inner.punct('&');
                        allowed_inner.group(Delimiter::Bracket, |allowed_slice| {
                            for (idx, (ident, _)) in self.iter_fields().enumerate() {
                                if idx != 0 {
                                    allowed_slice.punct(',');
                                }
                                allowed_slice.extend(ident);
                            }
                        });
                    });
                } else {
                    // no fixed values, implement a range
                    variant_inner
                        .push_parsed(format!(
                            "bincode::error::AllowedEnumVariants::Range {{ min: 0, max: {} }}",
                            self.variants.len() - 1
                        ))
                        .unwrap();
                }
            })
        });
    }

    pub fn generate_decode(self, generator: &mut Generator) -> Result<()> {
        let enum_name = generator.target_name().to_string();

        if generator.has_lifetimes() {
            // enum has a lifetime, implement BorrowDecode

            generator.impl_for_with_de_lifetime("bincode::de::BorrowDecode<'__de>")
                .unwrap()
                .generate_fn("borrow_decode")
                .with_generic("D", ["bincode::de::BorrowDecoder<'__de>"])
                .with_arg("mut decoder", "D")
                .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {
                    fn_builder
                        .push_parsed("let variant_index = <u32 as bincode::de::Decode>::decode(&mut decoder)?;").unwrap();
                    fn_builder.push_parsed("match variant_index").unwrap();
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                        for (mut variant_index, variant) in self.iter_fields() {
                            // idx => Ok(..)
                            if variant_index.len() > 1 {
                                variant_case.push_parsed("x if x == ").unwrap();
                                variant_case.extend(variant_index);
                            } else {
                                variant_case.push(variant_index.remove(0));
                            }
                            variant_case.puncts("=>");
                            variant_case.ident_str("Ok");
                            variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                                // Self::Variant { }
                                // Self::Variant { 0: ..., 1: ... 2: ... },
                                // Self::Variant { a: ..., b: ... c: ... },
                                variant_case_body.ident_str("Self");
                                variant_case_body.puncts("::");
                                variant_case_body.ident(variant.name.clone());

                                variant_case_body.group(Delimiter::Brace, |variant_body| {
                                    let is_tuple = matches!(variant.fields, Fields::Tuple(_));
                                    for (idx, field) in variant.fields.names().into_iter().enumerate() {
                                        if is_tuple {
                                            variant_body.lit_usize(idx);
                                        } else {
                                            variant_body.ident(field.unwrap_ident().clone());
                                        }
                                        variant_body.punct(':');
                                        variant_body.push_parsed("bincode::de::BorrowDecode::borrow_decode(&mut decoder)?,").unwrap();
                                    }
                                });
                            });
                            variant_case.punct(',');
                        }

                        // invalid idx
                        self.invalid_variant_case(&enum_name, variant_case);
                    });
                }).unwrap();
        } else {
            // enum has no lifetimes, implement Decode
            generator.impl_for("bincode::de::Decode")
            .unwrap()
                .generate_fn("decode")
                .with_generic("D", ["bincode::de::Decoder"])
                .with_arg("mut decoder", "D")
                .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
                .body(|fn_builder| {
                    fn_builder
                        .push_parsed("let variant_index = <u32 as bincode::de::Decode>::decode(&mut decoder)?;").unwrap();
                    fn_builder.push_parsed("match variant_index").unwrap();
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                        for (mut variant_index, variant) in self.iter_fields() {
                            // idx => Ok(..)
                            if variant_index.len() > 1 {
                                variant_case.push_parsed("x if x == ").unwrap();
                                variant_case.extend(variant_index);
                            } else {
                                variant_case.push(variant_index.remove(0));
                            }
                            variant_case.puncts("=>");
                            variant_case.ident_str("Ok");
                            variant_case.group(Delimiter::Parenthesis, |variant_case_body| {
                                // Self::Variant { }
                                // Self::Variant { 0: ..., 1: ... 2: ... },
                                // Self::Variant { a: ..., b: ... c: ... },
                                variant_case_body.ident_str("Self");
                                variant_case_body.puncts("::");
                                variant_case_body.ident(variant.name.clone());

                                variant_case_body.group(Delimiter::Brace, |variant_body| {
                                    let is_tuple = matches!(variant.fields, Fields::Tuple(_));
                                    for (idx, field) in variant.fields.names().into_iter().enumerate() {
                                        if is_tuple {
                                            variant_body.lit_usize(idx);
                                        } else {
                                            variant_body.ident(field.unwrap_ident().clone());
                                        }
                                        variant_body.punct(':');
                                        variant_body.push_parsed("bincode::de::Decode::decode(&mut decoder)?,").unwrap();
                                    }
                                });
                            });
                            variant_case.punct(',');
                        }

                        // invalid idx
                        self.invalid_variant_case(&enum_name, variant_case);
                    });
                }).unwrap();
        }

        Ok(())
    }
}

struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
    last_val: Option<(Literal, u32)>,
}

impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (Vec<TokenTree>, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.idx;
        let variant = self.variants.get(self.idx)?;
        self.idx += 1;

        let tokens = if let Fields::Integer(lit) = &variant.fields {
            let tree = TokenTree::Literal(lit.clone());
            self.last_val = Some((lit.clone(), 0));
            vec![tree]
        } else if let Some((lit, add)) = self.last_val.as_mut() {
            *add += 1;
            vec![
                TokenTree::Literal(lit.clone()),
                TokenTree::Punct(Punct::new('+', Spacing::Alone)),
                TokenTree::Literal(Literal::u32_suffixed(*add)),
            ]
        } else {
            vec![TokenTree::Literal(Literal::u32_suffixed(idx as u32))]
        };

        Some((tokens, variant))
    }
}
