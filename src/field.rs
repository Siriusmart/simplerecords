use std::{fmt::Display, hash::Hash};

use crate::Signature;

#[derive(Debug, Clone)]
/// Represents a typed field in a record.
pub enum Field {
    /// Case sensitive string.
    ///
    /// ```
    /// "alice" != "Alice"
    /// ```
    IStr(String),
    /// Case insensitive string.
    ///
    /// ```
    /// "alice" == "Alice"
    /// ```
    UStr { original: String, lower: String },
    /// Single character
    Char(char),
    /// 32 bit floating number
    F32(f32),
    /// 64 bit floating number
    F64(f64),
    /// 8 bit signed integer
    I8(i8),
    /// 16 bit signed integer
    I16(i16),
    /// 32 bit signed integer
    I32(i32),
    /// 64 bit signed integer
    I64(i64),
    /// 8 bit unsigned integer
    U8(u8),
    /// 16 bit unsigned integer
    U16(u16),
    /// 32 bit unsigned integer
    U32(u32),
    /// 64 bit unsigned integer
    U64(u64),
    /// Boolean value
    Bool(bool),
}

impl Field {
    /// Escape a string according to the specified quotation symbol
    pub fn escape_str(quote: char, s: &str) -> String {
        s.chars()
            .map(|c| match c {
                '\\' => "\\\\".to_string(),
                ':' => "\\:".to_string(),
                _ if c == quote => format!("\\{c}"),
                _ => c.to_string(),
            })
            .fold(String::new(), |mut acc, s| {
                acc.push_str(&s);
                acc
            })
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IStr(s) => f.write_fmt(format_args!("\"{}\"", Self::escape_str('\'', s))),
            Self::UStr { original, .. } => {
                f.write_fmt(format_args!("\"{}\"", Self::escape_str('\'', original)))
            }
            Self::Char(c) => f.write_fmt(format_args!("'{c}'")),
            Self::F32(n) => f.write_fmt(format_args!("{n}")),
            Self::F64(n) => f.write_fmt(format_args!("{n}")),
            Self::I8(i) => f.write_fmt(format_args!("{i}")),
            Self::I16(i) => f.write_fmt(format_args!("{i}")),
            Self::I32(i) => f.write_fmt(format_args!("{i}")),
            Self::I64(i) => f.write_fmt(format_args!("{i}")),
            Self::U8(u) => f.write_fmt(format_args!("{u}")),
            Self::U16(u) => f.write_fmt(format_args!("{u}")),
            Self::U32(u) => f.write_fmt(format_args!("{u}")),
            Self::U64(u) => f.write_fmt(format_args!("{u}")),
            Self::Bool(b) => f.write_fmt(format_args!("{b}")),
        }
    }
}

impl Field {
    /// Converts istr or ustr to &str
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::IStr(s) | Self::UStr { original: s, .. } => Some(s),
            _ => None,
        }
    }

    /// Converts char to char
    pub fn as_char(&self) -> Option<char> {
        match self {
            Self::Char(c) => Some(*c),
            _ => None,
        }
    }

    /// Converts f32 to f32
    pub fn as_f32(&self) -> Option<f32> {
        match self {
            Self::F32(f) => Some(*f),
            _ => None,
        }
    }

    /// Converts f32 or f64 to f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F32(f) => Some(*f as f64),
            Self::F64(f) => Some(*f),
            _ => None,
        }
    }

    /// Converts i8 to i8
    pub fn as_i8(&self) -> Option<i8> {
        match self {
            Self::I8(i) => Some(*i),
            _ => None,
        }
    }

    /// Converts i8 or i16 to i16
    pub fn as_i16(&self) -> Option<i16> {
        match self {
            Self::I8(i) => Some(*i as i16),
            Self::I16(i) => Some(*i),
            _ => None,
        }
    }

    /// Converts i8, i16 or i32 to i32
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            Self::I8(i) => Some(*i as i32),
            Self::I16(i) => Some(*i as i32),
            Self::I32(i) => Some(*i),
            _ => None,
        }
    }

    /// Converts i8, i16, i32 or i64 to i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Self::I8(i) => Some(*i as i64),
            Self::I16(i) => Some(*i as i64),
            Self::I32(i) => Some(*i as i64),
            Self::I64(i) => Some(*i),
            _ => None,
        }
    }

    /// Converts u8 to u8
    pub fn as_u8(&self) -> Option<u8> {
        match self {
            Self::U8(u) => Some(*u),
            _ => None,
        }
    }

    /// Converts u8 or u16 to u16
    pub fn as_u16(&self) -> Option<u16> {
        match self {
            Self::U8(u) => Some(*u as u16),
            Self::U16(u) => Some(*u),
            _ => None,
        }
    }

    /// Converts u8, u16 or u32 to u32
    pub fn as_u32(&self) -> Option<u32> {
        match self {
            Self::U8(u) => Some(*u as u32),
            Self::U16(u) => Some(*u as u32),
            Self::U32(u) => Some(*u),
            _ => None,
        }
    }

    /// Converts u8, u16, u32 or u64 to u64
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Self::U8(u) => Some(*u as u64),
            Self::U16(u) => Some(*u as u64),
            Self::U32(u) => Some(*u as u64),
            Self::U64(u) => Some(*u),
            _ => None,
        }
    }

    /// Converts bool to bool
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }
}

impl Field {
    /// Converts type to the most generic type.
    /// - f32 -> f64
    /// - i8, i16, i32 -> i64
    /// - u8, u16, u32 -> u64
    /// - ustr -> istr
    pub fn upcast(&self) -> Self {
        match self {
            Self::UStr { original, .. } | Self::IStr(original) => Self::IStr(original.clone()),
            Self::Char(c) => Self::Char(*c),
            Self::F32(f) => Self::F64(*f as f64),
            Self::F64(f) => Self::F64(*f),
            Self::I8(i) => Self::I64(*i as i64),
            Self::I16(i) => Self::I64(*i as i64),
            Self::I32(i) => Self::I64(*i as i64),
            Self::I64(i) => Self::I64(*i),
            Self::U8(u) => Self::U64(*u as u64),
            Self::U16(u) => Self::U64(*u as u64),
            Self::U32(u) => Self::U64(*u as u64),
            Self::U64(u) => Self::U64(*u),
            Self::Bool(b) => Self::Bool(*b),
        }
    }

    /// Converts type value to type signature.
    pub fn sig(&self) -> Signature {
        match self {
            Self::IStr(_) => Signature::IStr,
            Self::UStr { .. } => Signature::UStr,
            Self::Char(_) => Signature::Char,
            Self::F32(_) => Signature::F64,
            Self::F64(_) => Signature::F64,
            Self::I8(_) => Signature::I64,
            Self::I16(_) => Signature::I64,
            Self::I32(_) => Signature::I64,
            Self::I64(_) => Signature::I64,
            Self::U8(_) => Signature::U64,
            Self::U16(_) => Signature::U64,
            Self::U32(_) => Signature::U64,
            Self::U64(_) => Signature::U64,
            Self::Bool(_) => Signature::Bool,
        }
    }
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::IStr(a) => match other {
                Self::IStr(b) => a == b,
                _ => false,
            },
            Self::UStr { lower: a, .. } => match other {
                Self::UStr { lower: b, .. } => a == b,
                _ => false,
            },
            Self::Char(a) => match other {
                Self::Char(b) => a == b,
                _ => false,
            },
            Self::F32(a) => match other {
                Self::F32(b) => a == b,
                _ => false,
            },
            Self::F64(a) => match other {
                Self::F64(b) => a == b,
                _ => false,
            },
            Self::I8(a) => match other {
                Self::I8(b) => a == b,
                _ => false,
            },
            Self::I16(a) => match other {
                Self::I16(b) => a == b,
                _ => false,
            },
            Self::I32(a) => match other {
                Self::I32(b) => a == b,
                _ => false,
            },
            Self::I64(a) => match other {
                Self::I64(b) => a == b,
                _ => false,
            },
            Self::U8(a) => match other {
                Self::U8(b) => a == b,
                _ => false,
            },
            Self::U16(a) => match other {
                Self::U16(b) => a == b,
                _ => false,
            },
            Self::U32(a) => match other {
                Self::U32(b) => a == b,
                _ => false,
            },
            Self::U64(a) => match other {
                Self::U64(b) => a == b,
                _ => false,
            },
            Self::Bool(a) => match other {
                Self::Bool(b) => a == b,
                _ => false,
            },
        }
    }
}
impl Eq for Field {}
impl Hash for Field {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::IStr(s) => s.hash(state),
            Self::UStr { lower, .. } => lower.hash(state),
            Self::Char(c) => c.hash(state),
            Self::F32(f) => f.to_be_bytes().hash(state),
            Self::F64(f) => f.to_be_bytes().hash(state),
            Self::I8(i) => i.hash(state),
            Self::I16(i) => i.hash(state),
            Self::I32(i) => i.hash(state),
            Self::I64(i) => i.hash(state),
            Self::U8(u) => u.hash(state),
            Self::U16(u) => u.hash(state),
            Self::U32(u) => u.hash(state),
            Self::U64(u) => u.hash(state),
            Self::Bool(b) => b.hash(state),
        }
    }
}
