use super::FieldAttribute;
use virtue::generate::{FnSelfArg, Generator, StreamBuilder};
use virtue::parse::{EnumVariant, Fields};
use virtue::prelude::*;

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
            .impl_for("bincode::enc::Encode")?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints.push_constraint(g, "bincode::enc::Encode").unwrap();
                }
            })
            .generate_fn("encode")
            .with_generic("E", ["bincode::enc::Encoder"])
            .with_self_arg(FnSelfArg::RefSelf)
            .with_arg("mut encoder", "E")
            .with_return_type("core::result::Result<(), bincode::error::EncodeError>")
            .body(|fn_body| {
                fn_body.ident_str("match");
                fn_body.ident_str("self");
                fn_body.group(Delimiter::Brace, |match_body| {
                    if self.variants.is_empty() {
                        self.encode_empty_enum_case(match_body)?;
                    }
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
                                Ok(())
                            })?;
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
                            body.push_parsed("<u32 as bincode::enc::Encode>::encode")?;
                            body.group(Delimiter::Parenthesis, |args| {
                                args.punct('&');
                                args.group(Delimiter::Parenthesis, |num| {
                                    num.extend(variant_index);
                                    Ok(())
                                })?;
                                args.punct(',');
                                args.push_parsed("&mut encoder")?;
                                Ok(())
                            })?;
                            body.punct('?');
                            body.punct(';');
                            // If we have any fields, encode them all one by one
                            for field_name in variant.fields.names() {
                                if field_name.attributes().has_attribute(FieldAttribute::WithSerde)? {
                                    body.push_parsed(format!(
                                        "bincode::enc::Encode::encode(&bincode::serde::Compat({}), &mut encoder)?;",
                                        field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                    ))?;
                                } else {
                                    body.push_parsed(format!(
                                        "bincode::enc::Encode::encode({}, &mut encoder)?;",
                                        field_name.to_string_with_prefix(TUPLE_FIELD_PREFIX),
                                    ))
                                    ?;
                                }
                            }
                            body.push_parsed("Ok(())")?;
                            Ok(())
                        })?;
                        match_body.punct(',');
                    }
                    Ok(())
                })?;
                Ok(())
            })?;
        Ok(())
    }

    /// If we're encoding an empty enum, we need to add an empty case in the form of:
    /// `_ => core::unreachable!(),`
    fn encode_empty_enum_case(&self, builder: &mut StreamBuilder) -> Result {
        builder.push_parsed("_ => core::unreachable!()").map(|_| ())
    }

    /// Build the catch-all case for an int-to-enum decode implementation
    fn invalid_variant_case(&self, enum_name: &str, result: &mut StreamBuilder) -> Result {
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
            err_inner.push_parsed("bincode::error::DecodeError::UnexpectedVariant")?;
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
                    variant_inner.push_parsed("bincode::error::AllowedEnumVariants::Allowed")?;
                    variant_inner.group(Delimiter::Parenthesis, |allowed_inner| {
                        allowed_inner.punct('&');
                        allowed_inner.group(Delimiter::Bracket, |allowed_slice| {
                            for (idx, (ident, _)) in self.iter_fields().enumerate() {
                                if idx != 0 {
                                    allowed_slice.punct(',');
                                }
                                allowed_slice.extend(ident);
                            }
                            Ok(())
                        })?;
                        Ok(())
                    })?;
                } else {
                    // no fixed values, implement a range
                    variant_inner.push_parsed(format!(
                        "bincode::error::AllowedEnumVariants::Range {{ min: 0, max: {} }}",
                        self.variants.len() - 1
                    ))?;
                }
                Ok(())
            })?;
            Ok(())
        })?;
        Ok(())
    }

    pub fn generate_decode(&self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_borrow_decode

        let enum_name = generator.target_name().to_string();

        generator
            .impl_for("bincode::Decode")?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints.push_constraint(g, "bincode::enc::Decode").unwrap();
                }
            })
            .generate_fn("decode")
            .with_generic("D", ["bincode::de::Decoder"])
            .with_arg("mut decoder", "D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
            .body(|fn_builder| {
                if self.variants.is_empty() {
                    fn_builder.push_parsed("core::result::Result::Err(bincode::error::DecodeError::EmptyEnum { type_name: core::any::type_name::<Self>() })")?;
                } else {
                    fn_builder
                        .push_parsed(
                            "let variant_index = <u32 as bincode::Decode>::decode(&mut decoder)?;",
                        )?;
                    fn_builder.push_parsed("match variant_index")?;
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                        for (mut variant_index, variant) in self.iter_fields() {
                            // idx => Ok(..)
                            if variant_index.len() > 1 {
                                variant_case.push_parsed("x if x == ")?;
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
                                        if field.attributes().has_attribute(FieldAttribute::WithSerde)? {
                                            variant_body
                                                .push_parsed("<bincode::serde::Compat<_> as bincode::Decode>::decode(&mut decoder)?.0,")?;
                                        } else {
                                            variant_body
                                                .push_parsed("bincode::Decode::decode(&mut decoder)?,")?;
                                        }
                                    }
                                    Ok(())
                                })?;
                                Ok(())
                            })?;
                            variant_case.punct(',');
                        }

                        // invalid idx
                        self.invalid_variant_case(&enum_name, variant_case)
                    })?;
                }
                Ok(())
            })?;
        Ok(())
    }

    pub fn generate_borrow_decode(self, generator: &mut Generator) -> Result<()> {
        // Remember to keep this mostly in sync with generate_decode

        let enum_name = generator.target_name().to_string();

        generator.impl_for_with_lifetimes("bincode::de::BorrowDecode", &["__de"])?
            .modify_generic_constraints(|generics, where_constraints| {
                for g in generics.iter_generics() {
                    where_constraints.push_constraint(g, "bincode::enc::BorrowDecode").unwrap();
                }
            })
            .generate_fn("borrow_decode")
            .with_generic("D", ["bincode::de::BorrowDecoder<'__de>"])
            .with_arg("mut decoder", "D")
            .with_return_type("core::result::Result<Self, bincode::error::DecodeError>")
            .body(|fn_builder| {
                if self.variants.is_empty() {
                    fn_builder.push_parsed("core::result::Result::Err(bincode::error::DecodeError::EmptyEnum { type_name: core::any::type_name::<Self>() })")?;
                } else {
                    fn_builder
                        .push_parsed("let variant_index = <u32 as bincode::Decode>::decode(&mut decoder)?;")?;
                    fn_builder.push_parsed("match variant_index")?;
                    fn_builder.group(Delimiter::Brace, |variant_case| {
                        for (mut variant_index, variant) in self.iter_fields() {
                            // idx => Ok(..)
                            if variant_index.len() > 1 {
                                variant_case.push_parsed("x if x == ")?;
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
                                        if field.attributes().has_attribute(FieldAttribute::WithSerde)? {
                                            variant_body
                                                .push_parsed("<bincode::serde::BorrowCompat<_> as bincode::BorrowDecode>::borrow_decode(&mut decoder)?.0,")?;
                                        } else {
                                            variant_body.push_parsed("bincode::de::BorrowDecode::borrow_decode(&mut decoder)?,")?;
                                        }
                                    }
                                    Ok(())
                                })?;
                                Ok(())
                            })?;
                            variant_case.punct(',');
                        }

                        // invalid idx
                        self.invalid_variant_case(&enum_name, variant_case)
                    })?;
                }
                Ok(())
            })?;
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
