#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::Cell;
use range::Range;
use piston_meta::{ MetaData, Rule };

/// Returns rules for parsing meta rules.
pub fn meta_rules() -> Vec<(Rc<String>, Rule)> {
    use std::rc::Rc;
    use piston_meta::*;

    let opt: Rc<String> = Rc::new("optional".into());
    let inv: Rc<String> = Rc::new("inverted".into());
    let prop: Rc<String> = Rc::new("property".into());
    let any: Rc<String> = Rc::new("any_characters".into());
    let seps: Rc<String> = Rc::new("[]{}():.!?\"".into());

    // 1."string" [..seps!("name") ":" w? t?("text")]
    let string_rule = Rule::Sequence(Sequence {
        debug_id: 1000,
        args: vec![
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 1001,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("name".into()))
            }),
            Rule::Token(Token {
                debug_id: 1002,
                text: Rc::new(":".into()),
                inverted: false,
                property: None
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 1003,
                optional: true,
            }),
            Rule::Text(Text {
                debug_id: 1004,
                allow_empty: true,
                property: Some(Rc::new("text".into())),
            })
        ]
    });

    // 2 "node" [$"id" w! t!"name" w! @"rule""rule"]
    let node_rule = Rule::Sequence(Sequence {
        debug_id: 2000,
        args: vec![
            Rule::Number(Number {
                debug_id: 2001,
                allow_underscore: false,
                property: Some(Rc::new("id".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2002,
                optional: false,
            }),
            Rule::Text(Text {
                debug_id: 2003,
                allow_empty: false,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2004,
                optional: false,
            }),
            Rule::Node(Node {
                debug_id: 2005,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    // rule 3:"set" {t!("value") ..seps!("ref")}
    let set_rule = Rule::Select(Select {
        debug_id: 3003,
        args: vec![
            Rule::Text(Text {
                debug_id: 3004,
                allow_empty: false,
                property: Some(Rc::new("value".into())),
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 3005,
                any_characters: seps.clone(),
                optional: false,
                property: Some(Rc::new("ref".into())),
            })
        ]
    });

    // 4."opt" {"?"(opt) "!"(!opt)}
    let opt_rule = Rule::Select(Select {
        debug_id: 4000,
        args: vec![
            Rule::Token(Token {
                debug_id: 4001,
                text: Rc::new("?".into()),
                inverted: false,
                property: Some(opt.clone())
            }),
            Rule::Token(Token {
                debug_id: 4002,
                text: Rc::new("!".into()),
                inverted: true,
                property: Some(opt.clone())
            }),
        ]
    });

    // 5."number" ["$" ?("_"("underscore")) ?(@"set"("name"))]
    let number_rule = Rule::Sequence(Sequence {
        debug_id: 5000,
        args: vec![
            Rule::Token(Token {
                debug_id: 5001,
                text: Rc::new("$".into()),
                inverted: false,
                property: None,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 5002,
                rule: Rule::Token(Token {
                    debug_id: 5003,
                    text: Rc::new("_".into()),
                    inverted: false,
                    property: Some(Rc::new("underscore".into()))
                })
            })),
            Rule::Optional(Box::new(Optional {
                debug_id: 5004,
                rule: Rule::Node(Node {
                    debug_id: 5005,
                    name: Rc::new("set".into()),
                    property: Some(Rc::new("property".into())),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 6."text" ["t" {"?"("allow_empty") "!"(!"allow_empty")} ?(@"set"(prop))]
    let text_rule = Rule::Sequence(Sequence {
        debug_id: 6000,
        args: vec![
            Rule::Token(Token {
                debug_id: 6001,
                text: Rc::new("t".into()),
                inverted: false,
                property: None,
            }),
            Rule::Select(Select {
                debug_id: 6002,
                args: vec![
                    Rule::Token(Token {
                        debug_id: 6003,
                        text: Rc::new("?".into()),
                        inverted: false,
                        property: Some(Rc::new("allow_empty".into())),
                    }),
                    Rule::Token(Token {
                        debug_id: 6004,
                        text: Rc::new("!".into()),
                        inverted: true,
                        property: Some(Rc::new("allow_empty".into())),
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6005,
                rule: Rule::Node(Node {
                    debug_id: 6006,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            })),
        ]
    });

    // 7::"reference" ["@" t!"name" ?(@"set"prop)]
    let reference_rule = Rule::Sequence(Sequence {
        debug_id: 7000,
        args: vec![
            Rule::Token(Token {
                debug_id: 7001,
                text: Rc::new("@".into()),
                inverted: false,
                property: None,
            }),
            Rule::Text(Text {
                debug_id: 7002,
                allow_empty: false,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7003,
                rule: Rule::Node(Node {
                    debug_id: 7004,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None)
                })
            }))
        ]
    });

    // 8."sequence" ["[" w? s!.(w!) {@"rule"("rule")} "]"]
    let sequence_rule = Rule::Sequence(Sequence {
        debug_id: 8000,
        args: vec![
            Rule::Token(Token {
                debug_id: 8001,
                text: Rc::new("[".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8002,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 8003,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 8004,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 8005,
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None)
                })
            })),
            Rule::Token(Token {
                debug_id: 8006,
                text: Rc::new("]".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 9."select" ["{" w? s!.(w!) {@"rule"("rule")} "}"]
    let select_rule = Rule::Sequence(Sequence {
        debug_id: 9000,
        args: vec![
            Rule::Token(Token {
                debug_id: 9001,
                text: Rc::new("{".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 9002,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 9003,
                optional: false,
                allow_trail: true,
                by: Rule::Whitespace(Whitespace {
                    debug_id: 9004,
                    optional: false,
                }),
                rule: Rule::Node(Node {
                    debug_id: 9005,
                    name: Rc::new("rule".into()),
                    property: Some(Rc::new("rule".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Token(Token {
                debug_id: 9006,
                text: Rc::new("}".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 10."separated_by" ["s" @"opt" ?("."("allow_trail"))
    //   "(" w? @"rule"("by") w? ")" w? "{" w? @"rule"("rule") w? "}"]
    let separated_by_rule = Rule::Sequence(Sequence {
        debug_id: 10000,
        args: vec![
            Rule::Token(Token {
                debug_id: 10001,
                text: Rc::new("s".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 10002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 10003,
                rule: Rule::Token(Token {
                    debug_id: 10004,
                    text: Rc::new(".".into()),
                    inverted: false,
                    property: Some(Rc::new("allow_trail".into())),
                })
            })),
            Rule::Token(Token {
                debug_id: 10004,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10005,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 10006,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("by".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10007,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10008,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10009,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10010,
                text: Rc::new("{".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10011,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 10012,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 10013,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 10014,
                text: Rc::new("}".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    // 11 "token" [@"set""text" ?([?("!"(inv)) @"set"prop])]
    let token_rule = Rule::Sequence(Sequence {
        debug_id: 11000,
        args: vec![
            Rule::Node(Node {
                debug_id: 11001,
                name: Rc::new("set".into()),
                property: Some(Rc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 11002,
                rule: Rule::Sequence(Sequence {
                    debug_id: 11003,
                    args: vec![
                        Rule::Optional(Box::new(Optional {
                            debug_id: 11006,
                            rule: Rule::Token(Token {
                                debug_id: 11007,
                                text: Rc::new("!".into()),
                                inverted: false,
                                property: Some(inv.clone()),
                            })
                        })),
                        Rule::Node(Node {
                            debug_id: 11009,
                            name: Rc::new("set".into()),
                            property: Some(prop.clone()),
                            index: Cell::new(None),
                        })
                    ]
                })
            })),
        ]
    });

    // 12 "optional" ["?" @"rule""rule"]
    let optional_rule = Rule::Sequence(Sequence {
        debug_id: 12001,
        args: vec![
            Rule::Token(Token {
                debug_id: 12002,
                text: Rc::new("?".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 12004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    // 13."whitespace" ["w" @"opt"]
    let whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 13000,
        args: vec![
            Rule::Token(Token {
                debug_id: 13001,
                text: Rc::new("w".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 13002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            })
        ]
    });

    // 14."until_any_or_whitespace" [".." @"set"(any) @"opt" ?(@"set"(prop))]
    let until_any_or_whitespace_rule = Rule::Sequence(Sequence {
        debug_id: 14001,
        args: vec![
            Rule::Token(Token {
                debug_id: 14002,
                text: Rc::new("..".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 14003,
                name: Rc::new("set".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 14004,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 14005,
                rule: Rule::Node(Node {
                    debug_id: 14006,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 15."until_any" ["..." @"set"(any) @"opt" ?(@"set"(prop)) ]
    let until_any_rule = Rule::Sequence(Sequence {
        debug_id: 15000,
        args: vec![
            Rule::Token(Token {
                debug_id: 15001,
                text: Rc::new("...".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 15002,
                name: Rc::new("set".into()),
                property: Some(any.clone()),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 15003,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 15004,
                rule: Rule::Node(Node {
                    debug_id: 15005,
                    name: Rc::new("set".into()),
                    property: Some(prop.clone()),
                    index: Cell::new(None),
                })
            }))
        ]
    });

    // 16."repeat" ["r" @"opt" "(" @"rule"("rule") ")"]
    let repeat_rule = Rule::Sequence(Sequence {
        debug_id: 16000,
        args: vec![
            Rule::Token(Token {
                debug_id: 16001,
                text: Rc::new("r".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16002,
                name: Rc::new("opt".into()),
                property: None,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 16003,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Node(Node {
                debug_id: 16004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 16005,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 17."lines" ["l(" w? @"rule"("rule") w? ")"]
    let lines_rule = Rule::Sequence(Sequence {
        debug_id: 17000,
        args: vec![
            Rule::Token(Token {
                debug_id: 17001,
                text: Rc::new("l(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 17002,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 17003,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 17004,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 17005,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            })
        ]
    });

    // 18."rule" {5("number") 6("text") 7("reference") 8("sequence")
    //   9("select") 10("separated_by") 11("token") 12("optional")
    //   13("whitespace") 14("until_any_or_whitespace") 15("until_any")
    //   16("repeat") 17("lines")}
    let rule_rule = Rule::Select(Select {
        debug_id: 18000,
        args: vec![
            Rule::Node(Node {
                debug_id: 18009,
                name: Rc::new("whitespace".into()),
                property: Some(Rc::new("whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18009,
                name: Rc::new("until_any_or_whitespace".into()),
                property: Some(Rc::new("until_any_or_whitespace".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18010,
                name: Rc::new("until_any".into()),
                property: Some(Rc::new("until_any".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18012,
                name: Rc::new("lines".into()),
                property: Some(Rc::new("lines".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18011,
                name: Rc::new("repeat".into()),
                property: Some(Rc::new("repeat".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18001,
                name: Rc::new("number".into()),
                property: Some(Rc::new("number".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18002,
                name: Rc::new("text".into()),
                property: Some(Rc::new("text".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18003,
                name: Rc::new("reference".into()),
                property: Some(Rc::new("reference".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18004,
                name: Rc::new("sequence".into()),
                property: Some(Rc::new("sequence".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18005,
                name: Rc::new("select".into()),
                property: Some(Rc::new("select".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18006,
                name: Rc::new("separated_by".into()),
                property: Some(Rc::new("separated_by".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18007,
                name: Rc::new("token".into()),
                property: Some(Rc::new("token".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18008,
                name: Rc::new("optional".into()),
                property: Some(Rc::new("optional".into())),
                index: Cell::new(None),
            }),
        ]
    });

    // 19::"document" [l(@"string""string") l(@"node""node") w?]
    let document_rule = Rule::Sequence(Sequence {
        debug_id: 19000,
        args: vec![
            Rule::Lines(Box::new(Lines {
                debug_id: 19001,
                rule: Rule::Node(Node {
                    debug_id: 19002,
                    name: Rc::new("string".into()),
                    property: Some(Rc::new("string".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Lines(Box::new(Lines {
                debug_id: 19002,
                rule: Rule::Node(Node {
                    debug_id: 19003,
                    name: Rc::new("node".into()),
                    property: Some(Rc::new("node".into())),
                    index: Cell::new(None),
                })
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 19004,
                optional: true,
            })
        ]
    });

    let rules = vec![
        (Rc::new("string".into()), string_rule),
        (Rc::new("node".into()), node_rule),
        (Rc::new("set".into()), set_rule),
        (Rc::new("opt".into()), opt_rule),
        (Rc::new("number".into()), number_rule),
        (Rc::new("text".into()), text_rule),
        (Rc::new("reference".into()), reference_rule),
        (Rc::new("sequence".into()), sequence_rule),
        (Rc::new("select".into()), select_rule),
        (Rc::new("separated_by".into()), separated_by_rule),
        (Rc::new("token".into()), token_rule),
        (Rc::new("optional".into()), optional_rule),
        (Rc::new("whitespace".into()), whitespace_rule),
        (Rc::new("until_any_or_whitespace".into()), until_any_or_whitespace_rule),
        (Rc::new("until_any".into()), until_any_rule),
        (Rc::new("repeat".into()), repeat_rule),
        (Rc::new("lines".into()), lines_rule),
        (Rc::new("rule".into()), rule_rule),
        (Rc::new("document".into()), document_rule),
    ];
    update_refs(&rules);
    rules
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

/// Converts meta data to rules.
pub fn convert_meta_data_to_rules(mut data: &[(Range, MetaData)])
-> Result<Vec<(Rc<String>, Rule)>, ()> {
    use piston_meta::*;

    fn update(range: Range, data: &mut &[(Range, MetaData)], offset: &mut usize) {
        let next_offset = range.next_offset();
        *data = &data[next_offset - *offset..];
        *offset = next_offset;
    }

    fn start_node(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<Range, ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::StartNode(ref n) if &**n == name => {
                Ok(Range::new(offset, 1))
            }
            _ => Err(())
        }
    }

    fn end_node(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<Range, ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::EndNode(ref n) if &**n == name => {
                Ok(Range::new(offset, 1))
            }
            _ => Err(())
        }
    }

    fn meta_string(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, Rc<String>), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::String(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), val.clone()))
            }
            _ => Err(())
        }
    }

    fn meta_f64(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, f64), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::F64(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), *val))
            }
            _ => Err(())
        }
    }

    fn meta_bool(name: &str, data: &[(Range, MetaData)], offset: usize)
    -> Result<(Range, bool), ()> {
        if data.len() == 0 { return Err(()); }
        match &data[0].1 {
            &MetaData::Bool(ref n, ref val) if &**n == name => {
                Ok((Range::new(offset, 1), *val))
            }
            _ => Err(())
        }
    }

    fn read_string(mut data: &[(Range, MetaData)], mut offset: usize)
    -> Result<(Range, (Rc<String>, Rc<String>)), ()> {
        let start_offset = offset;
        let range = try!(start_node("string", data, offset));
        update(range, &mut data, &mut offset);
        let mut name = None;
        let mut text = None;
        loop {
            if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("text", data, offset) {
                text = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok(range) = end_node("string", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else {
                return Err(())
            }
        }
        let name = match name {
            None => { return Err(()); }
            Some(x) => x
        };
        let text = match text {
            None => { return Err(()); }
            Some(x) => x
        };
        Ok((Range::new(start_offset, offset - start_offset), (name, text)))
    }

    fn read_sequence(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("sequence", data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node("sequence", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings
            ) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                println!("TEST {} sequence {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset), Rule::Sequence(Sequence {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn find_string(val: &str, strings: &[(Rc<String>, Rc<String>)]) -> Option<Rc<String>> {
        strings.iter().find(|&&(ref s, _)| &**s == val).map(|&(_, ref s)| s.clone())
    }

    fn read_set(property: &str, mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, Rc<String>), ()> {
        let start_offset = offset;
        let range = try!(start_node(property, data, offset));
        update(range, &mut data, &mut offset);
        let mut text = None;
        loop {
            if let Ok(range) = end_node(property, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("ref", data, offset) {
                update(range, &mut data, &mut offset);
                text = find_string(&val, strings);
            } else if let Ok((range, val)) = meta_string("value", data, offset) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else {
                return Err(())
            }
        }
        match text {
            None => Err(()),
            Some(text) => Ok((Range::new(start_offset, offset - start_offset), text))
        }
    }

    fn read_until_any_or_whitespace(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("until_any_or_whitespace", data, offset));
        update(range, &mut data, &mut offset);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("until_any_or_whitespace", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", data, offset, strings) {
                update(range, &mut data, &mut offset);
                any_characters = Some(val);
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} until_any_or_whitespace {:?}", offset, &data[0]);
                return Err(());
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: *debug_id,
                    any_characters: any,
                    optional: optional,
                    property: property
                })))
            }
            None => Err(())
        }
    }

    fn read_until_any(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("until_any", data, offset));
        update(range, &mut data, &mut offset);
        let mut any_characters = None;
        let mut optional = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("until_any", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("any_characters", data, offset, strings) {
                update(range, &mut data, &mut offset);
                any_characters = Some(val);
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} until_any {:?}", offset, &data[0]);
                return Err(());
            }
        }
        let optional = optional.unwrap_or(false);
        match any_characters {
            Some(any) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::UntilAny(UntilAny {
                    debug_id: *debug_id,
                    any_characters: any,
                    optional: optional,
                    property: property
                })))
            }
            None => Err(())
        }
    }

    fn read_token(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("token", data, offset));
        update(range, &mut data, &mut offset);

        let mut text = None;
        let mut property = None;
        let mut inverted = None;
        loop {
            if let Ok(range) = end_node("token", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("text", data, offset, strings) {
                update(range, &mut data, &mut offset);
                text = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else if let Ok((range, val)) = meta_bool("inverted", data, offset) {
                update(range, &mut data, &mut offset);
                inverted = Some(val);
            } else {
                println!("TEST {} token {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        let inverted = inverted.unwrap_or(false);
        match text {
            Some(text) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Token(Token {
                    debug_id: *debug_id,
                    text: text,
                    inverted: inverted,
                    property: property,
                })))
            }
            None => Err(())
        }
    }

    fn read_whitespace(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        let (range, optional) = try!(meta_bool("optional", data, offset));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("whitespace", data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Whitespace(Whitespace {
            debug_id: *debug_id,
            optional: optional,
        })))
    }

    fn read_text(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("text", data, offset));
        update(range, &mut data, &mut offset);
        let mut allow_empty = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("text", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } if let Ok((range, val)) = meta_bool("allow_empty", data, offset) {
                update(range, &mut data, &mut offset);
                allow_empty = Some(val);
            } if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} text {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        let allow_empty = allow_empty.unwrap_or(true);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Text(Text {
            debug_id: *debug_id,
            allow_empty: allow_empty,
            property: property,
        })))
    }

    fn read_number(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("number", data, offset));
        update(range, &mut data, &mut offset);

        let mut property = None;
        let mut underscore = None;
        loop {
            if let Ok(range) = end_node("number", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else if let Ok((range, val)) = meta_bool("underscore", data, offset) {
                update(range, &mut data, &mut offset);
                underscore = Some(val);
            } else {
                println!("TEST {} number {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        let underscore = underscore.unwrap_or(false);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Number(Number {
            debug_id: *debug_id,
            property: property,
            allow_underscore: underscore,
        })))
    }

    fn read_reference(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("reference", data, offset));
        update(range, &mut data, &mut offset);

        let mut name = None;
        let mut property = None;
        loop {
            if let Ok(range) = end_node("reference", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                name = Some(val);
            } else if let Ok((range, val)) = read_set("property", data, offset, strings) {
                update(range, &mut data, &mut offset);
                property = Some(val);
            } else {
                println!("TEST {} reference {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        match name {
            Some(name) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Node(Node {
                    debug_id: *debug_id,
                    name: name,
                    property: property,
                    index: Cell::new(None),
                })))
            }
            None => Err(())
        }
    }

    fn read_select(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("select", data, offset));
        update(range, &mut data, &mut offset);
        let mut args: Vec<Rule> = vec![];
        loop {
            if let Ok(range) = end_node("select", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings
            ) {
                update(range, &mut data, &mut offset);
                args.push(val);
            } else {
                println!("TEST {} select {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Select(Select {
            debug_id: *debug_id,
            args: args
        })))
    }

    fn read_optional(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("optional", data, offset));
        update(range, &mut data, &mut offset);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", data, offset, strings
        ));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("optional", data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Optional(Box::new(Optional {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_separated_by(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("separated_by", data, offset));
        update(range, &mut data, &mut offset);
        let mut optional = None;
        let mut allow_trail = None;
        let mut by = None;
        let mut rule = None;
        loop {
            if let Ok(range) = end_node("separated_by", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_bool("optional", data, offset) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else if let Ok((range, val)) = meta_bool("allow_trail", data, offset) {
                update(range, &mut data, &mut offset);
                allow_trail = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "by", data, offset, strings
            ) {
                update(range, &mut data, &mut offset);
                by = Some(val);
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings
            ) {
                update(range, &mut data, &mut offset);
                rule = Some(val);
            } else {
                println!("TEST {} separated_by {:?}", offset, &data[0].1);
                return Err(());
            }
        }
        let optional = optional.unwrap_or(true);
        let allow_trail = allow_trail.unwrap_or(true);
        match (by, rule) {
            (Some(by), Some(rule)) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::SeparatedBy(Box::new(SeparatedBy {
                    debug_id: *debug_id,
                    optional: optional,
                    allow_trail: allow_trail,
                    by: by,
                    rule: rule,
                }))))
            }
            _ => Err(())
        }
    }

    fn read_lines(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("lines", data, offset));
        update(range, &mut data, &mut offset);
        let (range, rule) = try!(read_rule(
            debug_id, "rule", data, offset, strings
        ));
        update(range, &mut data, &mut offset);
        let range = try!(end_node("lines", data, offset));
        update(range, &mut data, &mut offset);
        *debug_id += 1;
        Ok((Range::new(start_offset, offset - start_offset),
        Rule::Lines(Box::new(Lines {
            debug_id: *debug_id,
            rule: rule,
        }))))
    }

    fn read_repeat(
        debug_id: &mut usize,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node("repeat", data, offset));
        update(range, &mut data, &mut offset);
        let mut rule = None;
        let mut optional = None;
        loop {
            if let Ok(range) = end_node("repeat", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = read_rule(
                debug_id, "rule", data, offset, strings
            ) {
                update(range, &mut data, &mut offset);
                rule = Some(val);
            } else if let Ok((range, val)) = meta_bool(
                "optional", data, offset
            ) {
                update(range, &mut data, &mut offset);
                optional = Some(val);
            } else {
                println!("TEST {} repeat {:?}", offset, &data[0]);
                return Err(());
            }
        }
        match (rule, optional) {
            (Some(rule), Some(optional)) => {
                *debug_id += 1;
                Ok((Range::new(start_offset, offset - start_offset),
                Rule::Repeat(Box::new(Repeat {
                    debug_id: *debug_id,
                    rule: rule,
                    optional: optional,
                }))))
            }
            _ => Err(())
        }
    }

    fn read_rule(
        debug_id: &mut usize,
        property: &str,
        mut data: &[(Range, MetaData)],
        mut offset: usize,
        strings: &[(Rc<String>, Rc<String>)]
    ) -> Result<(Range, Rule), ()> {
        let start_offset = offset;
        let range = try!(start_node(property, data, offset));
        update(range, &mut data, &mut offset);

        let mut rule = None;
        if let Ok((range, val)) = read_sequence(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any_or_whitespace(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_until_any(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_token(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_whitespace(
            debug_id, data, offset
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_text(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_number(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_reference(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_select(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_optional(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_separated_by(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_lines(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        } else if let Ok((range, val)) = read_repeat(
            debug_id, data, offset, strings
        ) {
            update(range, &mut data, &mut offset);
            rule = Some(val);
        }

        if let Some(rule) = rule {
            let range = try!(end_node(property, data, offset));
            update(range, &mut data, &mut offset);
            Ok((Range::new(start_offset, offset - start_offset), rule))
        } else {
            println!("TEST {} rule {:?}", offset, &data[0].1);
            Err(())
        }
    }

    fn read_node(mut data: &[(Range, MetaData)], mut offset: usize,
    strings: &[(Rc<String>, Rc<String>)])
    -> Result<(Range, (Rc<String>, Rule)), ()> {
        let start_offset = offset;
        let range = try!(start_node("node", data, offset));
        update(range, &mut data, &mut offset);
        let mut id = None;
        let mut name = None;
        let mut rule = None;
        loop {
            if let Ok(range) = end_node("node", data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_f64("id", data, offset) {
                id = Some(val as usize);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = meta_string("name", data, offset) {
                name = Some(val);
                update(range, &mut data, &mut offset);
            } else if let Ok((range, val)) = read_rule(
                    &mut (id.unwrap_or(0) * 1000), "rule",
                    data, offset, strings
            ) {
                rule = Some(val);
                update(range, &mut data, &mut offset);
            } else {
                println!("TEST node {:?}", &data[0]);
                return Err(())
            }
        }
        match (name, rule) {
            (Some(name), Some(rule)) => {
                Ok((Range::new(start_offset, offset - start_offset), (name, rule)))
            }
            _ => Err(())
        }
    }

    let mut strings: Vec<(Rc<String>, Rc<String>)> = vec![];
    let mut offset: usize = 0;
    loop {
        if let Ok((range, val)) = read_string(data, offset) {
            strings.push(val);
            update(range, &mut data, &mut offset);
        } else {
            break;
        }
    }
    let mut res = vec![];
    loop {
        if let Ok((range, val)) = read_node(data, offset, &strings) {
            update(range, &mut data, &mut offset);
            res.push(val);
        } else if offset < data.len() {
            return Err(());
        } else {
            break;
        }
    }
    update_refs(&res);
    Ok(res)
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
        use std::fs::File;
        use std::io::Read;
        use piston_meta::*;

        for file in &files {
            let mut file_h = try!(File::open(file));
            let mut source = String::new();
            try!(file_h.read_to_string(&mut source));

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
        use piston_meta::parse;
        use std::fs::File;
        use std::io::Read;
        use std::path::PathBuf;

        let meta_rules = meta_rules();
        let syntax: PathBuf = "assets/syntax.txt".into();
        let mut file_h = File::open(syntax).unwrap();
        let mut source = String::new();
        file_h.read_to_string(&mut source).unwrap();
        let res = parse(&meta_rules, &source).unwrap();
        // print_meta_data(&res[410..430]);
        let rules = convert_meta_data_to_rules(&res).unwrap();
        println!("{:?}", rules);

        // let rules = rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, vec![
                "assets/bool.txt".into(),
                "assets/nat.txt".into(),
                "assets/option.txt".into(),
                "assets/string.rs".into(),
            ]) {
            use piston_meta::*;

            let mut std_err = ParseStdErr::new(&source);
            println!("file: {:?}", file);
            // println!("source {}", source);
            std_err.error(range, err);
            assert!(false);
        }
    }

    #[test]
    fn meta_syntax() {
        let rules = meta_rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, vec![
                "assets/self-syntax.txt".into(),
                "assets/syntax.txt".into(),
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
