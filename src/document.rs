use std::{collections::HashMap, path::PathBuf};

use crate::{Error, Filter, Pass, Record, RecordSet, Schema};

/// Represents a parsed simplerecords document
#[derive(Debug)]
pub struct Document(HashMap<String, RecordSet>);

impl Document {
    /// Load a document, uses file path with extension `.rules` if non is specified.
    ///
    /// ```
    /// let doc = Document::load("filename");
    /// ```
    pub fn load(path: impl Into<PathBuf>) -> Result<Self, Error> {
        let pass = Pass::load(&path.into(), "root", 0, true)?;
        let (schema, records) = pass.destruct();

        Ok(Self(RecordSet::parse(Schema::parse(schema)?, records)?))
    }

    /// Find and returns the first result matching filter, order is not preserved.
    /// - `None` represents a wildcard.
    /// - Filter length must match rule length.
    ///
    /// ```
    /// let filter = Filter::new(
    ///     "rulename".to_string(),
    ///     &[Some(Field::UStr("field1")), Some(Field::U16(123)), None, None]
    /// );
    ///
    /// let found = doc.find_one(filter)
    /// ```
    pub fn find_one(&self, filter: Filter) -> Result<Option<&Record>, Error> {
        match self.0.get(filter.record()) {
            Some(rs) => rs.find_one(filter),
            None => Err(Error::NoDefinition {
                label: filter.record().to_string(),
            }),
        }
    }

    /// Find and returns all results matching filter, order is not preserved.
    /// - `None` represents a wildcard.
    /// - Filter length must match rule length.
    ///
    /// ```
    /// let filter = Filter::new(
    ///     "rulename".to_string(),
    ///     &[Some(Field::UStr("field1")), Some(Field::U16(123)), None, None]
    /// );
    ///
    /// let found = doc.find(filter)
    /// ```
    pub fn find(&self, filter: Filter) -> Result<Vec<&Record>, Error> {
        match self.0.get(filter.record()) {
            Some(rs) => rs.find(filter),
            None => Err(Error::NoDefinition {
                label: filter.record().to_string(),
            }),
        }
    }
}
