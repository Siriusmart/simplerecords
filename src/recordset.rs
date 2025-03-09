// pub struct Records()

use std::collections::HashMap;

use crate::{Error, Field, Filter, ParseError, Record, Schema, SchemaOne};

#[derive(Debug)]
/// Represents all records of the same type.
pub struct RecordSet {
    tree: RecordTree,
    schema: SchemaOne,
}

impl RecordSet {
    /// Constructs new self.
    pub fn new(schema: SchemaOne) -> Self {
        Self {
            tree: RecordTree::Void,
            schema,
        }
    }

    /// Puts record into set.
    pub fn put(&mut self, record: Record) -> Option<Record> {
        match &mut self.tree {
            RecordTree::Void if record.args().is_empty() => {
                self.tree = RecordTree::Unit(record);
                None
            }
            RecordTree::Void => {
                self.tree = RecordTree::Index(HashMap::new());
                self.tree.put(record.clone().args(), record)
            }
            t => t.put(record.args().to_vec().as_slice(), record),
        }
    }

    /// Returns type schema.
    pub fn schema(&self) -> &SchemaOne {
        &self.schema
    }

    /// Parse record stream into collection of RecordSet.
    pub fn parse(
        schema: Schema,
        entries: Vec<(String, u32, String, Vec<String>)>,
    ) -> Result<HashMap<String, RecordSet>, Error> {
        let mut parsed_entries = schema.as_template();

        for (location, no, label, args) in entries {
            let replaced = match parsed_entries.get_mut(&label) {
                Some(recordset) => recordset.put(Record::new(
                    label,
                    location.clone(),
                    no,
                    recordset.schema().parse(
                        args.iter().map(String::as_str).collect(),
                        &location,
                        no,
                    )?,
                )),
                None => {
                    return Err(Error::ParseError {
                        location,
                        line: no,
                        reason: ParseError::NoDefinition { label },
                    })
                }
            };

            if let Some(duplicate) = replaced {
                return Err(Error::ParseError {
                    location,
                    line: no,
                    reason: ParseError::DuplicatedEntry {
                        first_appear: duplicate.location().to_string(),
                        line: duplicate.line(),
                        label: duplicate.label().to_string(),
                    },
                });
            }
        }

        Ok(parsed_entries)
    }

    /// Find and returns the first result matching filter, order is not preserved.
    pub fn find_one(&self, mut filter: Filter) -> Result<Option<&Record>, Error> {
        filter.apply(self.schema());

        if !self.schema().match_filter(&filter) {
            return Err(Error::FilterMismatch {
                expected: Box::new(self.schema.clone()),
                got: filter.clone(),
            });
        }

        Ok(self.tree.find_one(filter.args()))
    }

    /// Find and returns all results matching filter, order is not preserved.
    pub fn find(&self, mut filter: Filter) -> Result<Vec<&Record>, Error> {
        filter.apply(self.schema());

        if !self.schema().match_filter(&filter) {
            return Err(Error::FilterMismatch {
                expected: Box::new(self.schema.clone()),
                got: filter.clone(),
            });
        }

        Ok(self.tree.find(filter.args()))
    }
}

#[derive(Debug)]
enum RecordTree {
    Index(HashMap<Field, RecordTree>),
    Unit(Record),
    Void,
}

impl RecordTree {
    pub fn put(&mut self, args: &[Field], mut record: Record) -> Option<Record> {
        match self {
            Self::Index(map) => {
                let upcast = args[0].upcast();
                if let Some(entry) = map.get_mut(&upcast) {
                    entry.put(&args[1..], record)
                } else if args.len() == 1 {
                    map.insert(upcast, Self::Unit(record)).map(|u| match u {
                        Self::Unit(u) => u,
                        _ => unreachable!("wrong length"),
                    })
                } else {
                    let mut index = Self::Index(HashMap::new());
                    index.put(&args[1..], record);
                    map.insert(upcast, index);
                    None
                }
            }
            Self::Unit(r) => {
                std::mem::swap(r, &mut record);
                Some(record)
            }
            Self::Void => unreachable!("wrong length"),
        }
    }

    pub fn find_one(&self, filter: &[Option<Field>]) -> Option<&Record> {
        match self {
            Self::Void => None,
            Self::Unit(r) => Some(r),
            Self::Index(map) => {
                if let Some(field) = &filter[0] {
                    match map.get(field) {
                        Some(t) => t.find_one(&filter[1..]),
                        None => None,
                    }
                } else {
                    for t in map.values() {
                        let res = t.find_one(&filter[1..]);
                        if res.is_some() {
                            return res;
                        }
                    }

                    None
                }
            }
        }
    }

    pub fn find(&self, filter: &[Option<Field>]) -> Vec<&Record> {
        match self {
            Self::Void => Vec::new(),
            Self::Unit(r) => vec![r],
            Self::Index(map) => {
                if let Some(field) = &filter[0] {
                    match map.get(field) {
                        Some(t) => t.find(&filter[1..]),
                        None => Vec::new(),
                    }
                } else {
                    let mut res = Vec::new();
                    for t in map.values() {
                        res.append(&mut t.find(&filter[1..]));
                    }

                    res
                }
            }
        }
    }
}
