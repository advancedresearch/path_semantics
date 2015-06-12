extern crate math_notation;
extern crate piston_meta;

use piston_meta::parse;
use math_notation::*;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

fn main() {
    let rules = meta_rules();
    let self_syntax: PathBuf = "assets/self-syntax.txt".into();
    let mut file_h = File::open(self_syntax).unwrap();
    let mut source = String::new();
    file_h.read_to_string(&mut source).unwrap();
    let res = parse(&rules, &source).unwrap();
    print_meta_data(&res[140..160]);
    let rules = convert_meta_data_to_rules(&res);
}
