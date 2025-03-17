use std::fmt::Display;

use crate::{Filter, SchemaOne, Signature};

#[derive(Debug, PartialEq, Eq)]
/// Represents all possible errors that can occur.
pub enum Error {
    /// There is no file at file path.
    FileNotFound { path: String },
    /// Error when reading/writing file.
    IoError { reason: String },
    /// Error when parsing input.
    ParseError {
        location: String,
        line: u32,
        reason: ParseError,
    },
    /// Filter does not match record definition.
    FilterMismatch {
        expected: Box<SchemaOne>,
        got: Filter,
    },
    /// No definition for specified record type.
    NoDefinition { label: String },
}

#[derive(Debug, PartialEq, Eq)]
/// Represents all possible errors that can occur while parsing from file.
pub enum ParseError {
    /// Too few arguments for commands
    MissingArguments,
    /// Illegal record type name (cannot be `include`)
    IllegalName { label: String },
    /// Two entries with exactly the same values is not allowed
    DuplicatedEntry {
        first_appear: String,
        line: u32,
        label: String,
    },
    /// Two type definitions for the same name is not allowed
    DuplicatedDefinition {
        first_appear: String,
        line: u32,
        label: String,
    },
    /// Unrecognised data type in type definition
    UnrecognisedType { got: String },
    /// String is not closed at EOL
    UnclosedString,
    /// Multi-line comment is not closed at EOF
    UnclosedMultiLineComment,
    /// Argument cannot be parsed for reason
    IllegalArgument,
    /// Too many arguments for commands
    TooManyArguments,
    /// Cannot convert record field data to correct type
    TypeError {
        target: Signature,
        value: String,
        reason: String,
    },
    /// Record has incorrect number of fields
    ArgumentLengthMismatch { expected: usize, got: usize },
    /// File specified in `include` not found
    FileNotFound { path: String },
    /// No definition for specified record type.
    NoDefinition { label: String },
    /// Error when reading/writing file as specified by `input`
    IoError { reason: String },
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileNotFound { path } => f.write_fmt(format_args!("file not found at `{path}`")),
            Self::IoError { reason } => f.write_fmt(format_args!("io error: {reason}")),
            Self::ParseError {
                location,
                line,
                reason,
            } => f.write_fmt(format_args!("parse error at {location}@{line}: {reason}")),
            Self::FilterMismatch { expected, got } => f.write_fmt(format_args!(
                "filter mismatch: expected {expected}, got {got}"
            )),
            Self::NoDefinition { label } => {
                f.write_fmt(format_args!("no definition for `{label}`"))
            }
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingArguments => f.write_str("missing arguments"),
            Self::IllegalName { label } => f.write_fmt(format_args!("illegal name `{label}`")),
            Self::DuplicatedEntry {
                first_appear,
                line,
                label,
            } => f.write_fmt(format_args!(
                "duplicated entry with {first_appear}@{line} for `{label}`"
            )),
            Self::DuplicatedDefinition {
                first_appear,
                line,
                label,
            } => f.write_fmt(format_args!(
                "duplicated definition for {first_appear}@{line} for `{label}`"
            )),
            Self::UnrecognisedType { got } => {
                f.write_fmt(format_args!("unrecognised type `{got}`"))
            }
            Self::UnclosedString => f.write_str("unclosed string"),
            Self::UnclosedMultiLineComment => f.write_str("unclosed multi-line comment"),
            Self::IllegalArgument => f.write_str("illegal argument"),
            Self::TooManyArguments => f.write_str("too many arguments"),
            Self::TypeError {
                target,
                value,
                reason,
            } => f.write_fmt(format_args!(
                "cannot convert `{value}` to `{target}` ({reason})"
            )),
            Self::ArgumentLengthMismatch { expected, got } => f.write_fmt(format_args!(
                "argument length mismatch (expected {expected}, got {got})"
            )),
            Self::FileNotFound { path } => f.write_fmt(format_args!("file not found at `{path}`")),
            Self::NoDefinition { label } => {
                f.write_fmt(format_args!("definition not found for `{label}`"))
            }
            Self::IoError { reason } => f.write_fmt(format_args!("io error ({reason})")),
        }
    }
}

impl std::error::Error for Error {}
