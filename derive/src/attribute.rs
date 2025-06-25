use virtue::prelude::*;
use virtue::utils::{parse_tagged_attribute, ParsedAttribute};

pub struct ContainerAttributes {
    pub crate_name: String,
    pub bounds: Option<(String, Literal)>,
    pub decode_bounds: Option<(String, Literal)>,
    pub decode_context: Option<(String, Literal)>,
    pub borrow_decode_bounds: Option<(String, Literal)>,
    pub encode_bounds: Option<(String, Literal)>,
    pub maxsize_bounds: Option<(String, Literal)>,
}

impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::bincode".to_string(),
            bounds: None,
            decode_bounds: None,
            decode_context: None,
            encode_bounds: None,
            borrow_decode_bounds: None,
            maxsize_bounds: None,
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
                ParsedAttribute::Property(key, val) if key.to_string() == "bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "decode_bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.decode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "decode_context" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.decode_context =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "encode_bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.encode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val) if key.to_string() == "maxsize_bounds" => {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.maxsize_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Property(key, val)
                    if key.to_string() == "borrow_decode_bounds" =>
                {
                    let val_string = val.to_string();
                    if val_string.starts_with('"') && val_string.ends_with('"') {
                        result.borrow_decode_bounds =
                            Some((val_string[1..val_string.len() - 1].to_string(), val));
                    } else {
                        return Err(Error::custom_at("Should be a literal str", val.span()));
                    }
                }
                ParsedAttribute::Tag(i) => {
                    return Err(Error::custom_at("Unknown container attribute", i.span()))
                }
                ParsedAttribute::Property(key, _) => {
                    return Err(Error::custom_at("Unknown container attribute", key.span()))
                }
                _ => {}
            }
        }
        Ok(Some(result))
    }
}

#[derive(Default)]
pub struct FieldAttributes {
    pub max_len: Option<usize>,
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
                ParsedAttribute::Property(key, len_tokens) if key.to_string() == "maxlen" => {
                    result.max_len = Some(len_tokens.to_string().parse().map_err(|_| {
                        Error::custom_at("Invalid len expression", len_tokens.span())
                    })?)
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

#[derive(Default, Clone, Copy)]
pub enum SerializationKind {
    #[default]
    Default,

    WithSerde,
    WithMaxSize(usize),
}

impl SerializationKind {
    pub fn non_default_or(self, new_default: Self) -> Self {
        if let Self::Default = self {
            new_default
        } else {
            self
        }
    }
}

impl TryFrom<FieldAttributes> for SerializationKind {
    type Error = Error;
    fn try_from(value: FieldAttributes) -> std::result::Result<Self, Self::Error> {
        match value.max_len {
            Some(n) => {
                if value.with_serde {
                    Err(Error::custom(
                        "deriving both with_serde and maxlen is forbidden",
                    ))
                } else {
                    Ok(Self::WithMaxSize(n))
                }
            }
            None => {
                if value.with_serde {
                    Ok(Self::WithSerde)
                } else {
                    Ok(Self::Default)
                }
            }
        }
    }
}
