use std::path::Path;

use crate::{Document, Error};

#[derive(Default, Clone, PartialEq, Eq)]
/// Represents options when opening a Document
///
/// ```
/// let doc = Options::default()
///     // define data types
///     .with("scope types")
///     .with("user : ustr u8")
///     .with("perm : ustr istr bool")
///     // specify files to load from
///     .with("scope imports")
///     .with("include users")
///     .with("include permissions")
///     .open().unwrap();
/// ```
pub struct Options {
    init: String,
}

impl Options {
    /// Prepend a line to the init statement
    pub fn with(&mut self, line: &str) -> &mut Self {
        self.init.push_str(line);
        self.init.push('\n');
        self
    }

    /// Open a file with the specified statements
    pub fn open(&self) -> Result<Document, Error> {
        Document::load_as(&self.init, Path::new("_init"))
    }
}
