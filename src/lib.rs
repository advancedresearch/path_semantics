#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use std::path::PathBuf;
use std::rc::Rc;
use range::Range;
use piston_meta::{ MetaData, Rule };

pub mod interpreter;

/// Gets the syntax rules.
pub fn syntax_rules() -> Vec<(Rc<String>, Rule)> {
    use piston_meta::*;

    let meta_rules = bootstrap::rules();
    let source = include_str!("../assets/syntax.txt");
    let res = parse(&meta_rules, source).unwrap();
    bootstrap::convert(&res, &mut vec![]).unwrap()
}

/// Reads a file to a string.
pub fn file_to_string<P>(file: P) -> Result<String, std::io::Error>
    where P: AsRef<std::path::Path>
{
    use std::fs::File;
    use std::io::Read;

    let mut file_h = try!(File::open(file));
    let mut source = String::new();
    try!(file_h.read_to_string(&mut source));
    Ok(source)
}

/// Writes meta data as JSON.
pub fn json<W>(w: &mut W, data: &[(Range, MetaData)])
    where W: std::io::Write
{
    use std::cmp::{ min, max };

    let indent_offset = 0;

    // Start indention such that it balances off to zero.
    let starts = data.iter()
        .filter(|x| if let &(_, MetaData::StartNode(_)) = *x { true } else { false })
        .count() as u32;
    let ends = data.iter()
        .filter(|x| if let &(_, MetaData::EndNode(_)) = *x { true } else { false })
        .count() as u32;
    let mut indent: u32 = max(starts, ends) - min(starts, ends);
    let mut first = true;
    for (i, d) in data.iter().enumerate() {
        let is_end = if let &(_, MetaData::EndNode(_)) = d {
            indent -= 1;
            true
        } else { false };
        let is_next_end = if i < data.len() - 1 {
            match &data[i + 1] {
                &(_, MetaData::EndNode(_)) => false,
                _ => true
            }
        } else { true };
        let print_comma = !first && !is_end && is_next_end;
        if print_comma { writeln!(w, ","); } else if i != 0 { writeln!(w, ""); }
        first = false;
        for _ in (0 .. indent_offset + indent) {
            write!(w, " ");
        }
        match d {
            &(_, MetaData::StartNode(ref name)) => {
                first = true;
                write_json_string(w, name);
                write!(w, ":{}", "{");
                indent += 1;
            }
            &(_, MetaData::EndNode(_)) => {
                write!(w, "{}", "}");
            }
            &(_, MetaData::Bool(ref name, val)) => {
                write_json_string(w, name);
                write!(w, ":{}", val);
            }
            &(_, MetaData::F64(ref name, val)) => {
                write_json_string(w, name);
                write!(w, ":{}", val);
            }
            &(_, MetaData::String(ref name, ref val)) => {
                write_json_string(w, name);
                write!(w, ":");
                write_json_string(w, val);
            }
        }
    }
    writeln!(w, "");
}

/// Prints a JSON string.
pub fn write_json_string<W>(w: &mut W, val: &str) where W: std::io::Write {
    write!(w, "\"");
    for c in val.chars() {
        if c == '\\' {
            write!(w, "\\\\");
        } else if c == '\"' {
            write!(w, "\\\"");
        } else {
            write!(w, "{}", c);
        }
    }
    write!(w, "\"");
}

/// Prints read meta data.
pub fn print_meta_data(data: &[(Range, MetaData)]) {
    json(&mut std::io::stdout(), data);
}

/// Stores information about error occursing when parsing syntax.
pub enum SyntaxError {
    /// An io error occured.
    IoError(std::io::Error),
    /// A meta rule failed when parsing syntax.
    MetaError(PathBuf, String, Range, piston_meta::ParseError),
}

impl From<std::io::Error> for SyntaxError {
    fn from(error: std::io::Error) -> SyntaxError {
        SyntaxError::IoError(error)
    }
}

/// Stores information about mathematical functions.
pub struct Syntax {
    /// The source files.
    pub files: Vec<PathBuf>,
}

impl Syntax {
    /// Parses syntax.
    pub fn new(rules: &[(Rc<String>, Rule)], files: Vec<PathBuf>)
    -> Result<Syntax, SyntaxError> {
        use piston_meta::*;

        for file in &files {
            let source = try!(file_to_string(file));
            let res = parse(&rules, &source);
            match res {
                Ok(_) => {
                    /*
                    println!("TEST tokens");
                    for token in &tokenizer.tokens[s.0..] {
                        println!("{:?}", token.0);
                    }
                    */
                }
                Err((range, err)) => {
                    return Err(SyntaxError::MetaError(
                        file.into(),
                        source,
                        range,
                        err
                    ));

                }
            }
        }
        Ok(Syntax {
            files: files,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn syntax() {
        let rules = syntax_rules();

        // let rules = rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, vec![
                "assets/bool.txt".into(),
                "assets/nat.txt".into(),
                "assets/option.txt".into(),
                "assets/string.txt".into(),
                "assets/the-simpsons.txt".into(),
            ]) {
            use piston_meta::*;

            let mut std_err = ParseStdErr::new(&source);
            println!("file: {:?}", file);
            // println!("source {}", source);
            std_err.error(range, err);
            assert!(false);
        }
    }
}
