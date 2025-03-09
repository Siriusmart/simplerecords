use std::fmt::Display;

use crate::{Field, SchemaOne, Signature};

#[derive(Debug, PartialEq, Eq, Clone)]
/// Represents a search query.
pub struct Filter {
    record: String,
    args: Vec<Option<Field>>,
}

impl Filter {
    /// Return arguments represented
    pub fn args(&self) -> &[Option<Field>] {
        &self.args
    }

    /// Return record type represented
    pub fn record(&self) -> &str {
        &self.record
    }

    /// Constructs new self
    pub fn new(record: String, args: &[Option<Field>]) -> Self {
        Self {
            record,
            args: args
                .iter()
                .map(|arg| arg.as_ref().map(|a| a.upcast()))
                .collect(),
        }
    }

    /// Apply a type definition to convert all istr to ustr where appropriate.
    pub fn apply(&mut self, schema: &SchemaOne) {
        for (arg, sig) in self.args.iter_mut().zip(schema.sig()) {
            if let Some(arg) = arg {
                if sig == &Signature::UStr {
                    match arg {
                        Field::IStr(s) => {
                            *arg = Field::UStr {
                                original: s.clone(),
                                lower: s.to_lowercase(),
                            }
                        }
                        _ => unreachable!("wrong type"),
                    }
                }
            }
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{} {}",
            self.record,
            &self
                .args
                .iter()
                .map(|arg| {
                    arg.as_ref()
                        .map(|a| a.to_string())
                        .unwrap_or("*".to_string())
                })
                .collect::<Vec<_>>()
                .join(" "),
        ))
    }
}
