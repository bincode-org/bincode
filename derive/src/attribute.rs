use virtue::prelude::*;
use virtue::utils::{parse_tagged_attribute, ParsedAttribute};

pub struct ContainerAttributes {
    pub crate_name: String,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::bincode".to_string(),
        }
    }
}

impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "bincode")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Self::default();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Property(key, val) if key.to_string() == "crate" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.crate_name = val_string[1..val_string.len() - 1].to_string();
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
        Ok(Some(result))
    }
}

#[derive(Default)]
pub struct FieldAttributes {
    pub with_serde: bool,
}

impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "bincode")? {
            Some(body) => body,
            None => return Ok(None),
        };
        let mut result = Self::default();
        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) if i.to_string() == "with_serde" => {
                    result.with_serde = true;
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
        Ok(Some(result))
    }
}
