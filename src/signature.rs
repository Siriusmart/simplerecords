use std::{fmt::Display, str::FromStr};

use crate::{Error, Field, ParseError};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Type signatures.
pub enum Signature {
    IStr,
    UStr,
    Char,
    F32,
    F64,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    Bool,
}

impl FromStr for Signature {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "istr" => Self::IStr,
            "ustr" => Self::UStr,
            "char" => Self::Char,
            "f32" => Self::F32,
            "f64" => Self::F64,
            "i8" => Self::I8,
            "i16" => Self::I16,
            "i32" => Self::I32,
            "i64" => Self::I64,
            "u8" => Self::U8,
            "u16" => Self::U16,
            "u32" => Self::U32,
            "u64" => Self::U64,
            "bool" => Self::Bool,
            _ => return Err(Self::Err::UnrecognisedType { got: s.to_string() }),
        })
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::IStr => "istr",
            Self::UStr => "ustr",
            Self::Char => "char",
            Self::F32 => "f32",
            Self::F64 => "f64",
            Self::I8 => "i8",
            Self::I16 => "i16",
            Self::I32 => "i32",
            Self::I64 => "i64",
            Self::U8 => "u8",
            Self::U16 => "u16",
            Self::U32 => "u32",
            Self::U64 => "u64",
            Self::Bool => "bool",
        })
    }
}

impl Signature {
    /// Parse string to typed field according to value of self.
    pub fn parse(&self, s: &str, location: &str, line: u32) -> Result<Field, Error> {
        Ok(match self {
            Self::IStr => Field::IStr(s.to_string()),
            Self::UStr => Field::UStr {
                original: s.to_string(),
                lower: s.to_lowercase(),
            },
            Self::Char if s.len() == 1 => Field::Char(s.chars().nth(0).unwrap()),
            Self::Char => {
                return Err(Error::ParseError {
                    location: location.to_string(),
                    line,
                    reason: ParseError::TypeError {
                        target: *self,
                        value: s.to_string(),
                        reason: "cannot convert to char".to_string(),
                    },
                })
            }
            Self::F32 => match s.parse() {
                Ok(n) => Field::F32(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::F64 => match s.parse() {
                Ok(n) => Field::F64(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::I8 => match s.parse() {
                Ok(n) => Field::I8(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::I16 => match s.parse() {
                Ok(n) => Field::I16(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::I32 => match s.parse() {
                Ok(n) => Field::I32(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::I64 => match s.parse() {
                Ok(n) => Field::I64(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::U8 => match s.parse() {
                Ok(n) => Field::U8(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::U16 => match s.parse() {
                Ok(n) => Field::U16(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::U32 => match s.parse() {
                Ok(n) => Field::U32(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::U64 => match s.parse() {
                Ok(n) => Field::U64(n),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
            Self::Bool => match s.parse() {
                Ok(b) => Field::Bool(b),
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: ParseError::TypeError {
                            target: *self,
                            value: s.to_string(),
                            reason: e.to_string(),
                        },
                    })
                }
            },
        })
    }
}
