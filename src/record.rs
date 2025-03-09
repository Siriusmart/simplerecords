use std::fmt::Display;

use crate::Field;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
/// Represents a single record.
pub struct Record {
    label: String,
    location: String,
    line: u32,
    args: Vec<Field>,
}

impl Record {
    /// Constructs new self.
    pub fn new(label: String, location: String, line: u32, args: Vec<Field>) -> Self {
        Self {
            label,
            location,
            line,
            args,
        }
    }

    /// Returns rule label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns fields.
    pub fn args(&self) -> &[Field] {
        &self.args
    }

    /// Returns file location.
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Returns line number.
    pub fn line(&self) -> u32 {
        self.line
    }
}

impl Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}@{}) {} {}",
            self.location,
            self.line,
            self.label,
            self.args
                .iter()
                .map(Field::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        ))
    }
}
