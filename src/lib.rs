#![deny(missing_docs)]

//! Bootstrapped meta rules for mathematical notation.

extern crate range;
extern crate piston_meta;

use std::path::PathBuf;
use std::rc::Rc;
use std::cell::Cell;
use range::Range;
use piston_meta::Rule;

/// Returns rules for parsing mathematical notation.
pub fn rules() -> (Rule, Vec<(Rc<String>, Rule)>) {
    use piston_meta::*;

    let separators: Rc<String> = Rc::new("()[]{},;:/*+-".into());

    let member_bracket = Rule::Optional(Box::new(Optional {
        debug_id: 100,
        rule: Rule::Sequence(Sequence {
            debug_id: 200,
            args: vec![
                Rule::Token(Token {
                    debug_id: 300,
                    text: Rc::new("[".into()),
                    inverted: false,
                    property: None,
                }),
                Rule::Select(Select {
                    debug_id: 400,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 500,
                            text: Rc::new(":".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                            debug_id: 600,
                            any_characters: separators.clone(),
                            optional: false,
                            property: None,
                        }),
                    ]
                }),
                Rule::Token(Token {
                    debug_id: 700,
                    text: Rc::new("]".into()),
                    inverted: false,
                    property: None,
                }),
            ],
        })
    }));

    let brackets_rule = Rule::SeparatedBy(Box::new(SeparatedBy {
        debug_id: 900,
        optional: true,
        allow_trail: false,
        rule: member_bracket,
        by: Rule::Whitespace(Whitespace {
            debug_id: 1000,
            optional: false,
        })
    }));

    let path_rule = Rule::Sequence(Sequence {
        debug_id: 1200,
        args: vec![
            Rule::Optional(Box::new(Optional {
                debug_id: 1300,
                rule: Rule::Token(Token {
                    debug_id: 1400,
                    text: Rc::new("::".into()),
                    inverted: false,
                    property: Some(Rc::new("root".into())),
                }),
            })),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 1500,
                optional: false,
                allow_trail: true,
                rule: Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 1600,
                    any_characters: separators.clone(),
                    optional: false,
                    property: Some(Rc::new("name".into())),
                }),
                by: Rule::Token(Token {
                    debug_id: 1700,
                    text: Rc::new("::".into()),
                    inverted: false,
                    property: None,
                }),
            })),
        ]
    });

    let arg_rule = Rule::Sequence(Sequence {
        debug_id: 1900,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("path".into()),
                property: Some(Rc::new("path".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 2000,
                rule: Rule::Node(Node {
                    name: Rc::new("repeated_arguments".into()),
                    property: Some(Rc::new("repeated_arguments".into())),
                    debug_id: 0,
                    index: Cell::new(None),
                })
            }))
        ]
    });

    let arguments = Rule::Sequence(Sequence {
        debug_id: 2200,
        args: vec![
            Rule::Token(Token {
                debug_id: 2300,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 2400,
                optional: true,
            }),
            Rule::SeparatedBy(Box::new(SeparatedBy {
                debug_id: 2500,
                optional: true,
                allow_trail: true,
                rule: Rule::Select(Select {
                    debug_id: 2600,
                    args: vec![
                        Rule::Number(Number {
                            debug_id: 2700,
                            allow_underscore: true,
                            property: None,
                        }),
                        Rule::Text(Text {
                            debug_id: 2800,
                            allow_empty: true,
                            property: None,
                        }),
                        Rule::Node(Node {
                            name: Rc::new("arguments".into()),
                            property: Some(Rc::new("arguments".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("member_lambda".into()),
                            property: Some(Rc::new("member_lambda".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("lambda".into()),
                            property: Some(Rc::new("lambda".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                        Rule::Node(Node {
                            name: Rc::new("arg".into()),
                            property: Some(Rc::new("arg".into())),
                            debug_id: 0,
                            index: Cell::new(None),
                        }),
                    ]
                }),
                by: Rule::Sequence(Sequence {
                    debug_id: 2900,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 3000,
                            text: Rc::new(",".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 3100,
                            optional: false,
                        }),
                    ],
                }),
            })),
            Rule::Whitespace(Whitespace {
                debug_id: 3200,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 3300,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let repeated_arguments = Rule::Repeat(Box::new(Repeat {
        debug_id: 3500,
        optional: false,
        rule: Rule::Node(Node {
            name: Rc::new("arguments".into()),
            property: Some(Rc::new("arguments".into())),
            debug_id: 0,
            index: Cell::new(None),
        }),
    }));

    let comment_rule = Rule::Sequence(Sequence {
        debug_id: 3700,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 22,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 3800,
                text: Rc::new("//".into()),
                inverted: false,
                property: None,
            }),
            Rule::UntilAny(UntilAny {
                debug_id: 3900,
                any_characters: Rc::new("\n".into()),
                optional: true,
                property: None,
            }),
        ],
    });

    let lambda = Rule::Sequence(Sequence {
        debug_id: 4100,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 4200,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 4300,
                rule: Rule::Sequence(Sequence {
                    debug_id: 4400,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 4500,
                            text: Rc::new("fn".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 4600,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 4700,
                any_characters: separators.clone(),
                optional: true,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 4800,
                optional: true,
            }),
            Rule::Node(Node {
                name: Rc::new("brackets".into()),
                property: Some(Rc::new("brackets".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("repeated_arguments".into()),
                property: Some(Rc::new("repeated_arguments".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 4900,
                optional: false,
            }),
            Rule::Token(Token {
                debug_id: 5000,
                text: Rc::new("->".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 5100,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 5200,
                optional: true,
            }),
        ]
    });

    let fn_rule = Rule::Sequence(Sequence {
        debug_id: 5400,
        args: vec![
            Rule::Optional(Box::new(Optional {
                debug_id: 5500,
                rule: Rule::Sequence(Sequence {
                    debug_id: 5600,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 5700,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 5800,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Node(Node {
                name: Rc::new("lambda".into()),
                property: Some(Rc::new("lambda".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 5900,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 6000,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6100,
                rule: Rule::Node(Node {
                    name: Rc::new("comment".into()),
                    property: Some(Rc::new("comment".into())),
                    debug_id: 0,
                    index: Cell::new(None),
                })
            })),
        ]
    });


    let use_rule = Rule::Sequence(Sequence {
        debug_id: 6300,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 6400,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 6500,
                rule: Rule::Sequence(Sequence {
                    debug_id: 6600,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 6700,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 6800,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Token(Token {
                debug_id: 6900,
                text: Rc::new("use".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 7000,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("path".into()),
                property: Some(Rc::new("path".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7100,
                rule: Rule::Token(Token {
                    debug_id: 7200,
                    text: Rc::new("*".into()),
                    inverted: false,
                    property: None,
                }),
            })),
            Rule::Token(Token {
                debug_id: 7300,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let module = Rule::Sequence(Sequence {
        debug_id: 7500,
        args: vec![
            Rule::Whitespace(Whitespace {
                debug_id: 7600,
                optional: true,
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7700,
                rule: Rule::Sequence(Sequence {
                    debug_id: 7800,
                    args: vec![
                        Rule::Token(Token {
                            debug_id: 7900,
                            text: Rc::new("pub".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 8000,
                            optional: true,
                        }),
                    ]
                }),
            })),
            Rule::Token(Token {
                debug_id: 8100,
                text: Rc::new("mod".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8200,
                optional: false,
            }),
            Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                debug_id: 8300,
                any_characters: separators.clone(),
                optional: true,
                property: Some(Rc::new("name".into())),
            }),
            Rule::Token(Token {
                debug_id: 8400,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let member_lambda = Rule::Sequence(Sequence {
        debug_id: 8600,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8700,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 8800,
                text: Rc::new(":".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 8900,
                optional: false,
            }),
            Rule::Node(Node {
                name: Rc::new("arg".into()),
                property: Some(Rc::new("arg".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
        ]
    });

    let member_rule = Rule::Sequence(Sequence {
        debug_id: 9100,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("member_lambda".into()),
                property: Some(Rc::new("member_lambda".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Token(Token {
                debug_id: 9200,
                text: Rc::new(";".into()),
                inverted: false,
                property: None,
            }),
        ]
    });

    let line_rule = Rule::Select(Select {
        debug_id: 9300,
        args: vec![
            Rule::Node(Node {
                name: Rc::new("comment".into()),
                property: Some(Rc::new("comment".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("use".into()),
                property: Some(Rc::new("use".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("module".into()),
                property: Some(Rc::new("module".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("member".into()),
                property: Some(Rc::new("member".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                name: Rc::new("fn".into()),
                property: Some(Rc::new("fn".into())),
                debug_id: 0,
                index: Cell::new(None),
            }),
        ]
    });

    let lines_rule = Rule::Lines(Box::new(Lines {
        debug_id: 9400,
        rule: line_rule,
    }));

    let refs: Vec<(Rc<String>, _)> = vec![
        (Rc::new("comment".into()), comment_rule),
        (Rc::new("use".into()), use_rule),
        (Rc::new("module".into()), module),
        (Rc::new("fn".into()), fn_rule),
        (Rc::new("lambda".into()), lambda),
        (Rc::new("arg".into()), arg_rule),
        (Rc::new("member".into()), member_rule),
        (Rc::new("member_lambda".into()), member_lambda),
        (Rc::new("brackets".into()), brackets_rule),
        (Rc::new("arguments".into()), arguments),
        (Rc::new("path".into()), path_rule),
        (Rc::new("repeated_arguments".into()), repeated_arguments),
    ];

    lines_rule.update_refs(&refs[..]);

    (lines_rule, refs)
}

/*
/// Returns rules for parsing meta rules.
pub fn meta_rules() -> piston_meta::Rule {
    use std::rc::Rc;
    use piston_meta::*;

    let opt: Rc<String> = Rc::new("optional".into());
    let inv: Rc<String> = Rc::new("inverted".into());
    let prop: Rc<String> = Rc::new("property".into());
    let any: Rc<String> = Rc::new("any_characters".into());
    let seps: Rc<String> = Rc::new("[]{}():".into());

    // 1."string" [..seps!("name") ":" w? t?("text")]
    let string_node = Node {
        debug_id: 1000,
        name: Rc::new("string".into()),
        rule: Rule::Sequence(Sequence {
            debug_id: 1001,
            args: vec![
                Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                    debug_id: 1002,
                    any_characters: seps.clone(),
                    optional: false,
                    property: Some(Rc::new("name".into())),
                }),
                Rule::Token(Token {
                    debug_id: 1003,
                    text: Rc::new(":".into()),
                    inverted: false,
                    property: None,
                }),
                Rule::Whitespace(Whitespace {
                    debug_id: 1004,
                    optional: true,
                }),
                Rule::Text(Text {
                    debug_id: 1005,
                    allow_empty: true,
                    property: Some(Rc::new("text".into()))
                })
            ]
        })
    };

    // 2."node" [$("id") "." t!("name") w! @"rule"]
    let node_node = Node {
        debug_id: 2000,
        name: Rc::new("node".into()),
        rule: Rule::Sequence(Sequence {
            debug_id: 2001,
            args: vec![
                Rule::Number(Number {
                    debug_id: 2002,
                    allow_underscore: false,
                    property: Some(Rc::new("id".into())),
                }),
                Rule::Token(Token {
                    debug_id: 2003,
                    text: Rc::new(".".into()),
                    inverted: false,
                    property: None,
                }),
                Rule::Text(Text {
                    debug_id: 2004,
                    allow_empty: false,
                    property: Some(Rc::new("name".into())),
                }),
                Rule::Whitespace(Whitespace {
                    debug_id: 2005,
                    optional: false,
                }),
                Rule::Node(NodeRef::Name(Rc::new("rule".into()), 2006)),
            ]
        })
    };

    // 3."set" ["(" w? {t!("value") ..seps!("ref")} w? ")"]
    let set_node = Node {
        debug_id: 3000,
        name: Rc::new("set".into()),
        rule: Rule::Sequence(Sequence {
            debug_id: 3001,
            args: vec![
                Rule::Token(Token {
                    debug_id: 3002,
                    text: Rc::new("(".into()),
                    inverted: false,
                    property: None,
                }),
                Rule::Whitespace(Whitespace {
                    debug_id: 3003,
                    optional: true,
                }),
                Rule::Select(Select {
                    debug_id: 3004,
                    args: vec![
                        Rule::Text(Text {
                            debug_id: 3005,
                            allow_empty: false,
                            property: Some(Rc::new("value".into())),
                        }),
                        Rule::UntilAnyOrWhitespace(UntilAnyOrWhitespace {
                            debug_id: 3006,
                            any_characters: seps.clone(),
                            optional: false,
                            property: Some(Rc::new("ref".into())),
                        })
                    ]
                }),
                Rule::Whitespace(Whitespace {
                    debug_id: 3007,
                    optional: true,
                }),
                Rule::Token(Token {
                    debug_id: 3008,
                    text: Rc::new(")".into()),
                    inverted: false,
                    property: None,
                })
            ]
        })
    };

    // 4."opt" {"?"(opt) "!"(!opt)}
    let opt_node = Node {
        debug_id: 4000,
        name: Rc::new("opt".into()),
        rule: Rule::Select(Select {
            debug_id: 4001,
            args: vec![
                Rule::Token(Token {
                    debug_id: 4002,
                    text: Rc::new("?".into()),
                    inverted: false,
                    property: Some(opt.clone())
                }),
                Rule::Token(Token {
                    debug_id: 4003,
                    text: Rc::new("!".into()),
                    inverted: true,
                    property: Some(opt.clone())
                })
            ]
        })
    };

    // 5."number" ["$" ?("_"("underscore")) ?(@"set"("name"))]
    let number_node = Node {
        debug_id: 5000,
        name: Rc::new("number".into()),
        rule: Rule::Sequence(Sequence {
            debug_id: 5001,
            args: vec![
                Rule::Token(Token {
                    debug_id: 5002,
                    text: Rc::new("$".into()),
                    inverted: false,
                    property: None
                }),
                Rule::Optional(Box::new(Optional {
                    debug_id: 5003,
                    rule: Rule::Token(Token {
                        debug_id: 5004,
                        text: Rc::new("_".into()),
                        inverted: false,
                        property: Some(Rc::new("underscore".into()))
                    })
                })),
                Rule::Optional(Box::new(Optional {
                    debug_id: 5005,
                    rule: Rule::Node(NodeRef::Name())
                }))
            ]
        })
    }
}
*/

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
    pub fn new(files: Vec<PathBuf>) -> Result<Syntax, SyntaxError> {
        use std::fs::File;
        use std::io::Read;
        use piston_meta::*;

        let (lines_rule, refs) = rules();

        for file in &files {
            let mut file_h = try!(File::open(file));
            let mut source = String::new();
            try!(file_h.read_to_string(&mut source));

            let chars: Vec<char> = source.chars().collect();
            let offset: usize = 0;
            let chars: &[char] = &chars;
            let mut tokenizer = Tokenizer::new();
            let s = TokenizerState::new();
            let res = lines_rule.parse(&mut tokenizer, &s, &chars, offset, &refs);
            match res {
                Ok((ok_range, _s, opt_error)) => {
                    /*
                    println!("TEST tokens");
                    for token in &tokenizer.tokens[s.0..] {
                        println!("{:?}", token.0);
                    }
                    */
                    if let Some((range, err)) = opt_error {
                        if ok_range.next_offset() < chars.len() {
                            return Err(SyntaxError::MetaError(
                                file.into(),
                                source,
                                range,
                                err
                            ));
                        }
                    }
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
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(vec![
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
}
