#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use piston_meta::Syntax;

pub mod interpreter;

/// Gets the syntax rules.
pub fn syntax_rules() -> Syntax {
    use piston_meta::*;

    let meta_rules = bootstrap::rules();
    let source = include_str!("../assets/syntax.txt");
    let mut res = vec![];
    parse(&meta_rules, source, &mut res).unwrap();
    bootstrap::convert(&res, &mut vec![]).unwrap()
}

#[cfg(test)]
mod tests {
    use piston_meta::*;

    #[test]
    fn test_syntax() {
        let syntax = "assets/syntax.txt";
        let _ = load_syntax_data2(syntax, "assets/bool.txt");
        let _ = load_syntax_data2(syntax, "assets/nat.txt");
        let _ = load_syntax_data2(syntax, "assets/option.txt");
        let _ = load_syntax_data2(syntax, "assets/string.txt");
        let _ = load_syntax_data2(syntax, "assets/the-simpsons.txt");
    }
}
