use std::{ffi::OsStr, fs, path::Path};

use crate::{Error, ParseError};

const QUOTES: &[char] = &['\'', '"', '`'];

/// Splits a file input into two streams.
/// - Definitions are parsed by the schema parser.
/// - Records are parsed by the records parser, after schema parsing has completed.
pub struct Pass {
    // location, line, label, signature
    schema: Vec<(String, u32, String, Vec<String>)>,
    // location, line, name, args
    records: Vec<(String, u32, String, Vec<String>)>,
}

impl Pass {
    /// Returns the internal types.
    #[allow(clippy::type_complexity)]
    pub fn destruct(
        self,
    ) -> (
        Vec<(String, u32, String, Vec<String>)>,
        Vec<(String, u32, String, Vec<String>)>,
    ) {
        (self.schema, self.records)
    }

    /// Remove comments from file content.
    pub fn clear_comment(s: &str) -> Result<String, (u32, ParseError)> {
        let mut cleared = String::new();

        let mut multi_escaped = false;
        let mut multi_tick = false;
        let mut multi_line = false;
        let mut line_escaped = false;
        let mut in_string = None;

        let mut escaped = false;

        let mut line_no = 1;

        for c in s.chars() {
            if multi_escaped {
                match c {
                    '*' => multi_tick = true,
                    '/' if multi_tick => {
                        multi_escaped = false;
                        multi_tick = false;
                        if multi_line {
                            multi_line = false;
                        } else {
                            cleared.push(' ');
                        }
                    }
                    '\n' => {
                        cleared.push('\n');
                        multi_line = true;
                        line_no += 1
                    }
                    _ => {}
                }

                continue;
            } else if line_escaped && c != '\n' {
                continue;
            } else {
                if c == '\\' {
                    escaped = !escaped;
                } else if escaped {
                    escaped = false;
                } else if let Some(quote) = in_string {
                    if c == quote {
                        in_string = None;
                    }
                } else {
                    match c {
                        '#' => {
                            line_escaped = true;
                            continue;
                        }
                        '/' => multi_tick = true,
                        '*' if multi_tick => {
                            multi_tick = false;
                            multi_escaped = true;
                            cleared.pop();
                            continue;
                        }
                        _ if QUOTES.contains(&c) => in_string = Some(c),
                        _ => {}
                    }
                }

                if c == '\n' {
                    if in_string.is_some() {
                        return Err((line_no, ParseError::UnclosedString));
                    }
                    multi_tick = false;
                    line_escaped = false;

                    line_no += 1;
                }

                cleared.push(c);
            }
        }

        if multi_escaped {
            return Err((line_no, ParseError::UnclosedMultiLineComment));
        }

        if in_string.is_some() {
            return Err((line_no, ParseError::UnclosedString));
        }

        Ok(cleared)
    }

    /// Split a line containing type definition by colon.
    pub fn split_col(s: &str) -> Option<(String, String)> {
        let mut in_string = None;

        let mut post_colon = false;

        let mut before = String::new();
        let mut after = String::new();

        let mut escaped = false;

        for c in s.chars() {
            if escaped {
                escaped = false
            } else if c == '\\' {
                escaped = !escaped
            } else if let Some(quote) = in_string {
                if quote == c {
                    in_string = None;
                }
            } else {
                match c {
                    ':' if !post_colon => {
                        post_colon = true;
                        continue;
                    }
                    _ if QUOTES.contains(&c) => in_string = Some(c),
                    _ => {}
                }
            }

            if post_colon {
                after.push(c);
            } else {
                before.push(c);
            }
        }

        if post_colon {
            Some((before, after))
        } else {
            None
        }
    }

    /// Split string into an argument list.
    pub fn split_args(s: &str) -> Result<Vec<String>, ParseError> {
        let mut buf = String::new();
        let mut args = Vec::new();

        let mut in_string = None;
        let mut just_ended_string = false;
        let mut escaped = false;

        for c in s.chars() {
            if c == '\\' {
                escaped = true;
                continue;
            } else if escaped {
                if c == ' ' && in_string.is_none() {
                    return Err(ParseError::IllegalArgument);
                }
                escaped = false
            } else if let Some(quote) = in_string {
                if quote == c {
                    in_string = None;
                    just_ended_string = true;
                    continue;
                }
            }
            match c {
                _ if in_string.is_some() => buf.push(c),
                ' ' => {
                    let push = buf.trim().to_string();
                    if push.is_empty() {
                        buf = push;
                        continue;
                    }
                    args.push(push);
                    buf = String::new();
                    just_ended_string = false;
                }
                _ if just_ended_string => return Err(ParseError::IllegalArgument),
                _ if QUOTES.contains(&c) => in_string = Some(c),
                _ => buf.push(c),
            }
        }

        if !buf.is_empty() {
            args.push(buf);
        }

        Ok(args)
    }

    /// Split file content into two streams.
    pub fn parse(file: &Path, s: &str) -> Result<Self, Error> {
        let filename = if file.extension() == Some(OsStr::new("rules")) {
            file.file_stem()
                .unwrap_or(OsStr::new("unnamed"))
                .to_string_lossy()
                .to_string()
        } else {
            file.to_string_lossy().to_string()
        };
        let s = match Self::clear_comment(s) {
            Ok(s) => s,
            Err((line, reason)) => {
                return Err(Error::ParseError {
                    location: filename,
                    line,
                    reason,
                })
            }
        };

        let mut scope = None;
        let mut schema = Vec::new();
        let mut records = Vec::new();

        for (mut line, no) in s.lines().zip(1_u32..) {
            line = line.trim();

            if line.is_empty() {
                continue;
            }

            if let Some((label, sig)) = Self::split_col(line) {
                let args = match Self::split_args(&sig) {
                    Ok(a) => a,
                    Err(e) => {
                        return Err(Error::ParseError {
                            location: filename,
                            line: no,
                            reason: e,
                        })
                    }
                };

                schema.push((
                    if let Some(scope) = &scope {
                        format!("{filename}<{scope}>")
                    } else {
                        filename.clone()
                    },
                    no,
                    label.trim().to_string(),
                    args,
                ));
                continue;
            }

            let words = match Self::split_args(line) {
                Ok(a) => a,
                Err(e) => {
                    return Err(Error::ParseError {
                        location: filename,
                        line: no,
                        reason: e,
                    })
                }
            };

            match words[0].to_lowercase().as_str() {
                "include" if words.len() == 1 => {
                    return Err(Error::ParseError {
                        location: filename,
                        line: no,
                        reason: ParseError::MissingArguments,
                    })
                }
                "include" if words.len() > 2 => {
                    return Err(Error::ParseError {
                        location: filename,
                        line: no,
                        reason: ParseError::TooManyArguments,
                    })
                }
                "include" => {
                    let new_path = file
                        .parent()
                        .unwrap_or(Path::new(""))
                        .join(words[1].as_str());
                    let mut loaded = Self::load(&new_path, &filename, no, false)?;
                    schema.append(&mut loaded.schema);
                    records.append(&mut loaded.records);
                }
                "scope" if words.len() == 1 => scope = None,
                "scope" if words.len() == 2 => scope = Some(words[1].clone()),
                "scope" => {
                    return Err(Error::ParseError {
                        location: filename,
                        line: no,
                        reason: ParseError::TooManyArguments,
                    })
                }
                label => records.push((
                    if let Some(scope) = &scope {
                        format!("{filename}<{scope}>")
                    } else {
                        filename.clone()
                    },
                    no,
                    label.to_string(),
                    words[1..].to_vec(),
                )),
            }
        }

        Ok(Self { schema, records })
    }

    /// Read and split a file into streams.
    pub fn load(file: &Path, source: &str, import_line: u32, root: bool) -> Result<Self, Error> {
        let to_load = if file.extension().is_some() {
            file.to_path_buf()
        } else {
            let new = file.with_extension("rules");
            if new.exists() {
                new
            } else {
                return Err(Error::ParseError {
                    location: source.to_string(),
                    line: import_line,
                    reason: ParseError::FileNotFound {
                        path: new.to_string_lossy().to_string(),
                    },
                });
            }
        };

        let s = match fs::read_to_string(&to_load) {
            Ok(s) => s,
            Err(e) if root => {
                return Err(Error::IoError {
                    reason: e.to_string(),
                })
            }
            Err(e) => {
                return Err(Error::ParseError {
                    location: source.to_string(),
                    line: import_line,
                    reason: ParseError::IoError {
                        reason: e.to_string(),
                    },
                })
            }
        };

        Self::parse(&to_load, &s)
    }
}
