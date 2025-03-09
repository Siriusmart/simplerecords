//! # Simple Records &emsp; [![Latest Version]][crates.io] [![Downloads]][crates.io] [![Stars]][github.com]
//! 
//! [Stars]: https://shields.io/github/stars/siriusmart/simplerecords?style=social
//! [Downloads]: https://img.shields.io/crates/d/simplerecords?label=crates.io%20downloads
//! [Latest Version]: https://img.shields.io/crates/v/simplerecords
//! [crates.io]: https://crates.io/crates/simplerecords
//! [github.com]: https://github.com/siriusmart/simplerecords
//! 
//! **Strongly typed text-based format for declarative configuration.**
//! 
//! ```
//! [dependencies]
//! simplerecords = "0.1"
//! ```
//! 
//! Here are some reasons to use *Simple Records*.
//! - **Human readable.** For large number of rules, records can be written in a table-like format.
//! - **Performant searching.** All records are indexed to be ready for search.
//! - **Multi-file support.** Records and definitions can be imported from any file for maximum flexibility.
//! 
//! ## Format specification
//! 
//! **Comments** are ignored.
//! - Single line: `#`
//! - Multi-line: `/* */`
//! ```text
//! #               username    IP          expiry
//! ```
//! 
//! **Definition** define the record *type*.
//! ```text
//! #               v--- unsigned string    v--- 64 bit unsigned integer
//! whitelist:      ustr        istr        u64
//! #                           ^--- signed string
//! ```
//! **Record** has a *type* and fields.
//! ```
//! whitelist       joe         127.0.0.1   123456
//! whitelist       bob         127.0.0.1   123457
//! whitelist       alice       127.0.0.3   123459
//! ```
//! **Import** additional rule files.
//! ```
//! include filename
//! ```
//! 
//! > The **default file extension** if none specified, is `*.rules`.
//! 
//! Definition and rules can be in **any file** and in **any order**, as long as it exists.
//! 
//! ## Usage
//! ```
//! use simplerecords::*;
//! ```
//! 1. Read a document.
//! ```
//! let doc = Document::read("whitelist") // reads from whitelist.rules
//! ```
//! 2. Create a search filter for `* '127.0.0.1' *`.
//! ```
//! let filter = Filter::new("whitelist", &[None, Some("127.0.0.1"), None])
//! ```
//! 3. Run the search.
//! ```
//! let found = doc.find(filter);
//! ```
//! 4. Output the results.
//! ```
//! (whitelist.rules@3) whitelist "joe" "127.0.0.1" 123456
//! (whitelist.rules@4) whitelist "bob" "127.0.0.1" 123457
//! ```
//! 
//! The example above can be found in [`src/examples`](https://github.com/Siriusmart/simplerecords/blob/master/examples/basic/src/main.rs).
//! 
//! ### Types
//! 
//! |Type|Description|
//! |---|---|
//! |char|A single character|
//! |i8|An integer value between -128 and 127|
//! |u8|An integer value between 0 and 255|
//! |i16|An integer value between -32k and 32k|
//! |u16|An integer value between 0 and 65k|
//! |i32|An integer value between -2.1B and 2.1B|
//! |u32|An integer value between 0 and 4.2B|
//! |i64|You get the idea|
//! |u64|You get the idea|
//! |f32|32 bit floating number.|
//! |f64|64 bit floating number.|
//! |istr|Case sensitive string.|
//! |ustr|Case insensitive string.|
//! 
//! ## Todo
//! 
//! All current features are **stable** and there will be no breaking changes.
//! 
//! New features will be added until this becomes a text-based database. Including
//! - Insert/delete/amending records.
//! - Value constraints

mod document;
mod error;
mod field;
mod filter;
mod pass;
mod record;
mod recordset;
mod schema;
mod signature;

pub use document::*;
pub use error::*;
pub use field::*;
pub use filter::*;
pub use pass::*;
pub use record::*;
pub use recordset::*;
pub use schema::*;
pub use signature::*;

#[cfg(test)]
mod tests;
