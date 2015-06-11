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

    update_refs(&lines_rule, &refs[..]);

    (lines_rule, refs)
}

/// Returns rules for parsing meta rules.
pub fn meta_rules() -> (Rule, Vec<(Rc<String>, Rule)>) {
    use std::rc::Rc;
    use piston_meta::*;

    let opt: Rc<String> = Rc::new("optional".into());
    let inv: Rc<String> = Rc::new("inverted".into());
    let prop: Rc<String> = Rc::new("property".into());
    let any: Rc<String> = Rc::new("any_characters".into());
    let seps: Rc<String> = Rc::new("[]{}():".into());

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

    // 2."node" [$("id") "." t!("name") w! @"rule"("rule")]
    let node_rule = Rule::Sequence(Sequence {
        debug_id: 2000,
        args: vec![
            Rule::Number(Number {
                debug_id: 2001,
                allow_underscore: false,
                property: Some(Rc::new("id".into())),
            }),
            Rule::Token(Token {
                debug_id: 2002,
                text: Rc::new(".".into()),
                inverted: false,
                property: None,
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

    // 3."set" ["(" w? {t!("value") ..seps!("ref")} w? ")"]
    let set_rule = Rule::Sequence(Sequence {
        debug_id: 3000,
        args: vec![
            Rule::Token(Token {
                debug_id: 3001,
                text: Rc::new("(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 3002,
                optional: true,
            }),
            Rule::Select(Select {
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
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 3004,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 3005,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
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
            })
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
                    property: Some(Rc::new("name".into())),
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

    // 7."reference" [{["@" t!("name")] $("id")} ?(@"set"(prop))]
    let reference_rule = Rule::Sequence(Sequence {
        debug_id: 7000,
        args: vec![
            Rule::Select(Select {
                debug_id: 7001,
                args: vec![
                    Rule::Sequence(Sequence {
                        debug_id: 7002,
                        args: vec![
                            Rule::Token(Token {
                                debug_id: 7003,
                                text: Rc::new("@".into()),
                                inverted: false,
                                property: None,
                            }),
                            Rule::Text(Text {
                                debug_id: 7004,
                                allow_empty: false,
                                property: Some(Rc::new("name".into())),
                            }),
                        ]
                    }),
                    Rule::Number(Number {
                        debug_id: 7005,
                        allow_underscore: false,
                        property: Some(Rc::new("id".into()))
                    })
                ]
            }),
            Rule::Optional(Box::new(Optional {
                debug_id: 7006,
                rule: Rule::Node(Node {
                    debug_id: 7007,
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

    // 11."token" [@"set"("text") ?(["(" w?
    //   {"!"(inv) "?"(!inv)} @"set"(prop) w? ")"])]
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
                        Rule::Token(Token {
                            debug_id: 11004,
                            text: Rc::new("(".into()),
                            inverted: false,
                            property: None,
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 11005,
                            optional: true,
                        }),
                        Rule::Select(Select {
                            debug_id: 11006,
                            args: vec![
                                Rule::Token(Token {
                                    debug_id: 11007,
                                    text: Rc::new("!".into()),
                                    inverted: false,
                                    property: Some(inv.clone()),
                                }),
                                Rule::Token(Token {
                                    debug_id: 11008,
                                    text: Rc::new("?".into()),
                                    inverted: true,
                                    property: Some(inv.clone()),
                                })
                            ]
                        }),
                        Rule::Node(Node {
                            debug_id: 11009,
                            name: Rc::new("set".into()),
                            property: Some(prop.clone()),
                            index: Cell::new(None),
                        }),
                        Rule::Whitespace(Whitespace {
                            debug_id: 11010,
                            optional: true,
                        }),
                        Rule::Token(Token {
                            debug_id: 11011,
                            text: Rc::new(")".into()),
                            inverted: false,
                            property: None,
                        })
                    ]
                })
            })),
        ]
    });

    // 12."optional" ["?(" w? @"rule"("rule") w? ")"]
    let optional_rule = Rule::Sequence(Sequence {
        debug_id: 12001,
        args: vec![
            Rule::Token(Token {
                debug_id: 12002,
                text: Rc::new("?(".into()),
                inverted: false,
                property: None,
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 12003,
                optional: true,
            }),
            Rule::Node(Node {
                debug_id: 12004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            }),
            Rule::Whitespace(Whitespace {
                debug_id: 12005,
                optional: true,
            }),
            Rule::Token(Token {
                debug_id: 12006,
                text: Rc::new(")".into()),
                inverted: false,
                property: None,
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
                debug_id: 18011,
                name: Rc::new("repeat".into()),
                property: Some(Rc::new("repeat".into())),
                index: Cell::new(None),
            }),
            Rule::Node(Node {
                debug_id: 18012,
                name: Rc::new("lines".into()),
                property: Some(Rc::new("lines".into())),
                index: Cell::new(None),
            }),
        ]
    });

    // [l(@"string"("string")) l(@"node"("node")) @"rule"("rule")]
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
            Rule::Node(Node {
                debug_id: 19004,
                name: Rc::new("rule".into()),
                property: Some(Rc::new("rule".into())),
                index: Cell::new(None),
            })
        ]
    });

    let refs = vec![
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
    ];
    (document_rule, refs)
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
    pub fn new(
        rules: &Rule,
        refs: &[(Rc<String>, Rule)],
        files: Vec<PathBuf>
    ) -> Result<Syntax, SyntaxError> {
        use std::fs::File;
        use std::io::Read;
        use piston_meta::*;

        for file in &files {
            let mut file_h = try!(File::open(file));
            let mut source = String::new();
            try!(file_h.read_to_string(&mut source));

            let res = parse(&rules, &refs, &source);
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
        let (rules, refs) = rules();
        if let Err(SyntaxError::MetaError(file, source, range, err))
            = Syntax::new(&rules, &refs, vec![
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
