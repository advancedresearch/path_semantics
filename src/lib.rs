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

/// Prints read meta data.
pub fn print_meta_data(data: &[(Range, MetaData)]) {
    for d in data {
        match &d.1 {
            &MetaData::StartNode(ref name) => {
                println!("start `{}`", name);
            }
            &MetaData::EndNode(ref name) => {
                println!("end `{}`", name);
            }
            &MetaData::F64(ref name, val) => {
                println!("{}: {}", name, val);
            }
            &MetaData::Bool(ref name, val) => {
                println!("{}: {}", name, val);
            }
            &MetaData::String(ref name, ref val) => {
                println!("{}: {}", name, val);
            }
        }
    }
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
