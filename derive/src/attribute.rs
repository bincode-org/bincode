use virtue::prelude::*;
use virtue::utils::{parse_tagged_attribute, ParsedAttribute};

#[derive(Default)]
pub struct ContainerAttributes(Vec<Container>);

#[non_exhaustive]
pub enum Container {
    CrateName(String),
}

impl ContainerAttributes {
    pub fn get_crate_name(&self) -> String {
        self.0
            .iter()
            // TODO: .filter_map when we have more Container entries
            .map(|a| {
                let Container::CrateName(n) = a;
                n.clone()
            })
            .next()
            .unwrap_or_else(|| "::bincode".to_string())
    }
}

impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "bincode")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Vec::new();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Property(key, val) if key.to_string() == "crate" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.push(Container::CrateName(
                            val_string[1..val_string.len() - 1].to_string(),
                        ));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Tag(i) => {
                    return Err(Error::custom_at("Unknown field attribute", i.span()))
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown field attribute", key.span()))
                }
                _ => {}
            }
        }
        Ok(Some(Self(result)))
    }
}

pub struct FieldAttributes(Vec<Field>);

impl FieldAttributes {
    pub fn has_with_serde(&self) -> bool {
        self.0.iter().any(|f| matches!(f, Field::WithSerde))
    }
}

#[non_exhaustive]
pub enum Field {
    WithSerde,
}

impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "bincode")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Vec::new();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) if i.to_string() == "with_serde" => {
                    result.push(Field::WithSerde);
                }
                ParsedAttribute::Tag(i) => {
                    return Err(Error::custom_at("Unknown field attribute", i.span()))
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown field attribute", key.span()))
                }
                _ => {}
            }
        }
        Ok(Some(Self(result)))
    }
}
