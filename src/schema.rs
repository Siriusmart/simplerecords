use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    str::FromStr,
};

use crate::{Error, Field, Filter, ParseError, RecordSet, Signature};

/// Represents a single type definition.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SchemaOne {
    label: String,
    location: String,
    line: u32,
    sig: Vec<Signature>,
}

impl Display for SchemaOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "({}@{}) {}: {}",
            self.location,
            self.line,
            self.label,
            if self.sig.is_empty() {
                "unit".to_string()
            } else {
                self.sig
                    .iter()
                    .map(Signature::to_string)
                    .collect::<Vec<_>>()
                    .join(" ")
            }
        ))
    }
}

impl SchemaOne {
    /// Parse a record into typed record.
    pub fn parse(&self, args: Vec<&str>, location: &str, line: u32) -> Result<Vec<Field>, Error> {
        if args.len() != self.sig.len() {
            return Err(Error::ParseError {
                location: location.to_string(),
                line,
                reason: ParseError::ArgumentLengthMismatch {
                    expected: self.sig.len(),
                    got: args.len(),
                },
            });
        }

        let mut out = Vec::with_capacity(args.len());

        for (arg, schem) in args.into_iter().zip(self.sig.iter()) {
            out.push(schem.parse(arg, location, line)?);
        }

        Ok(out)
    }

    /// Returns underlying label.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Returns location line number.
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Returns location file.
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Returns type signature list.
    pub fn sig(&self) -> &[Signature] {
        &self.sig
    }
}

impl SchemaOne {
    fn from_sig(
        label: String,
        location: String,
        line: u32,
        sig: &[String],
    ) -> Result<Self, ParseError> {
        let args = sig.iter().filter(|s| !s.is_empty()).collect::<Vec<_>>();

        let mut sig = Vec::with_capacity(args.len());

        for arg in args {
            sig.push(Signature::from_str(arg)?);
        }

        Ok(Self {
            label,
            location,
            line,
            sig,
        })
    }

    /// Check if filter is in correct type.
    pub fn match_filter(&self, filter: &Filter) -> bool {
        filter.args().len() == self.sig.len()
            && !filter
                .args()
                .iter()
                .zip(self.sig.iter())
                .any(|(f, s)| f.as_ref().is_some_and(|f| &f.sig() != s))
    }
}

// (location, schema)
/// Represents all the type definitions
pub struct Schema(HashMap<String, SchemaOne>);

// (location, line, label, schema)
impl Schema {
    /// Parse schema stream into self.
    pub fn parse(entries: Vec<(String, u32, String, Vec<String>)>) -> Result<Self, Error> {
        let mut parsed_entries = HashMap::with_capacity(entries.len());

        for (location, line, label, sig) in entries.into_iter() {
            if label.as_str() == "include" || label.contains(' ') {
                return Err(Error::ParseError {
                    location,
                    line,
                    reason: ParseError::IllegalName { label },
                });
            }

            let record = match SchemaOne::from_sig(label.clone(), location.clone(), line, &sig) {
                Ok(sig) => sig,
                Err(e) => {
                    return Err(Error::ParseError {
                        location: location.to_string(),
                        line,
                        reason: e,
                    })
                }
            };

            match parsed_entries.entry(label) {
                Entry::Vacant(en) => en.insert(record),
                Entry::Occupied(en) => {
                    let record = en.remove();
                    return Err(Error::ParseError {
                        location,
                        line,
                        reason: ParseError::DuplicatedDefinition {
                            first_appear: record.location().to_string(),
                            line: record.line(),
                            label: record.label().to_string(),
                        },
                    });
                }
            };
        }

        Ok(Self(parsed_entries))
    }

    /// Converts self to a blank collection of RecordSet.
    pub fn as_template(self) -> HashMap<String, RecordSet> {
        self.0
            .into_iter()
            .map(|(k, v)| (k, RecordSet::new(v)))
            .collect()
    }
}
