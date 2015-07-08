//! An interpreter implementation.

use piston_meta::*;
use range::Range;

/// Instructions.
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Op {
    /// Treats the next instruction as a function call.
    Call,
    /// Treats the next instruction as argument path.
    Path,
    /// Treats the next instruction as function path.
    FnPath,
    /// A function reference.
    FnRef(usize),
    /// An instruction reference.
    OpRef(usize),
    /// A variable reference.
    Var(usize),
    /// End of instructions.
    End,
}

/// Evaluates a program.
pub fn eval(fns: &[Op], ops: &[Op], st: &mut Vec<Op>) {
    let mut vars: Vec<Op> = Vec::new();
    let mut calls: Vec<usize> = Vec::new();
    let mut call = ops.len();
    'e: loop {
        let mut f = None;
        // Push instructions.
        for op in ops[0..call].iter().rev() {
            match op {
                &Op::Var(x) => {
                    // Find variable.
                    let mut v_index = None;
                    for i in (0 .. vars.len()).rev() {
                        if &vars[i] == &Op::Var(x) {
                            v_index = Some(i);
                            break;
                        }
                    }
                    let start = 1 + v_index.expect("Count not find variable index");
                    let end = start + vars[start..].iter()
                        .take_while(|&v| v != &Op::End).count();
                    // Push variable instructions.
                    for v in vars[start .. end].iter().rev() {
                        st.push(*v);
                    }
                }
                &Op::End => {
                    // Continue previous call.
                    call = match calls.pop() {
                        None => { return; }
                        Some(call) => call,
                    };
                    continue 'e;
                }
                &Op::Call => {
                    // Call a function.
                    call -= 1;
                    f = match st.last() {
                        Some(&Op::FnRef(f)) => Some(f),
                        x => { panic!("Expected `FnRef`, found `{:?}`", x); }
                    };
                    break;
                }
                x => {
                    // Push instruction.
                    call -= 1;
                    st.push(*x);
                }
            }
        }

        if let Some(mut f) = f {
            // The length of variable stack before pusing function variables.
            let var_len = vars.len();
            // Pattern match function.
            'f: loop {
                // An index of the instruction on the stack to match against.
                let mut j = st.len();
                for (i, fi) in fns[f..].iter().enumerate() {
                    match fi {
                        &Op::End => {
                            panic!("Function does not return a value");
                        }
                        &Op::OpRef(o) => {
                            // Remove values and call function.
                            st.truncate(j);
                            calls.push(call);
                            call = o + 1;
                            continue 'e;
                        }
                        _ => {}
                    }
                    if j == 0 {
                        panic!("Stack is shorter than function signature");
                    }
                    if let &Op::Var(x) = fi {
                        // The function signature contains a variable.

                        // Look for existing variable.
                        let mut existing_var = None;
                        for v in vars[var_len..].iter().rev() {
                            match v {
                                &Op::Var(y) if x == y => {
                                    existing_var = Some(y);
                                }
                                _ => {}
                            }
                        }

                        if let Some(k) = existing_var {
                            // Check for equality with existing variable.
                            let mut equal = true;
                            for v in &vars[k + 1..] {
                                match v {
                                    &Op::End => { break; }
                                    x if x == &st[j - 1] => { j -= 1; continue; }
                                    _ => { equal = false; break; }
                                }
                            }

                            if !equal {
                                // The pattern match failed.
                                // Try next function.
                                // Roll back variables.
                                vars.truncate(var_len);
                                for (k, fi) in fns[f + i..].iter().enumerate() {
                                    match fi {
                                        &Op::End | &Op::OpRef(_) => {
                                            f = f + i + k + 1;
                                            if f >= fns.len() {
                                                panic!("No function matched (end)");
                                            }
                                            continue 'f;
                                        }
                                        _ => {}
                                    }
                                }

                                panic!("Expected `End` or `OpRef` after function signature");
                            }
                        } else {
                            // Push new variable.
                            let mut count_down: usize = 1;
                            vars.push(Op::Var(x));
                            while count_down > 0 && j > 0 {
                                j -= 1;
                                vars.push(st[j]);
                                if let &Op::Path = &st[j] {
                                    count_down += 1;
                                } else {
                                    count_down -= 1;
                                }
                            }
                            vars.push(Op::End);
                        }
                        continue;
                    }
                    j -= 1;
                    if fi != &st[j] {
                        // The pattern match failed.
                        if i == 0 {
                            panic!("No function matched (failed to match name)");
                        }
                        // Try next function.
                        // Roll back variables.
                        vars.truncate(var_len);
                        for (k, fi) in fns[f + i..].iter().enumerate() {
                            match fi {
                                &Op::End | &Op::OpRef(_) => {
                                    f = f + i + k + 1;
                                    if f >= fns.len() {
                                        panic!("No function matched (end)");
                                    }
                                    continue 'f;
                                }
                                _ => {}
                            }
                        }

                        panic!("Expected `End` or `OpRef` after function signature");
                    }
                }
            }
        }
    }
}

/// Converts from meta data to function and instruction stack.
pub fn convert(_data: &[(Range, MetaData)]) -> (Vec<Op>, Vec<Op>) {
    let fns = vec![];
    let ops = vec![];
    // ::print_meta_data(&data);
    (fns, ops)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Op::*;

    #[test]
    fn not_true() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // not(bool) -> bool
            FnRef(6),                   // 6: not
            FnRef(0),                   // 7: bool,
            OpRef(1),                   // 8: -> bool
            // not([true] true) -> [false] false
            FnRef(6),                   // 9: not
            Path,
            FnRef(4),                   // 11: true
            FnRef(4),                   // 12: true
            OpRef(5),                   // 13: -> [false] false,
        ];

        let ops = vec![
            // bool
            End,
            FnRef(0),                   // 1: bool
            // [false] false
            End,
            Path,
            FnRef(2),                   // 4: false
            FnRef(2),                   // 5: false
            // call not([true] true)
            End,
            Call,
            FnRef(6),                   // 8: not
            Path,
            FnRef(4),                   // 10: true
            FnRef(4),                   // 11: true
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(2),   // false
            FnRef(2),
            Path
        ]);
    }

    #[test]
    fn not_false() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // not(bool) -> bool
            FnRef(6),                   // 6: not
            FnRef(0),                   // 7: bool,
            OpRef(1),                   // 8: -> bool
            // not([true] true) -> [false] false
            FnRef(6),                   // 9: not
            Path,
            FnRef(4),                   // 11: true
            FnRef(4),                   // 12: true
            OpRef(5),                   // 13: -> [false] false,
            // not([false] false) -> [true] true
            FnRef(6),                   // 14: not
            Path,
            FnRef(2),                   // 16: false
            FnRef(2),                   // 17: false
            OpRef(9),                   // 18: -> [true] true
        ];

        let ops = vec![
            // bool
            End,
            FnRef(0),                   // 1: bool
            // [false] false
            End,
            Path,
            FnRef(2),                   // 4: false
            FnRef(2),                   // 5: false
            // [true] true
            End,
            Path,
            FnRef(4),                   // 8: false
            FnRef(4),                   // 9: false
            // call not([true] true)
            End,
            Call,
            FnRef(6),                   // 8: not
            Path,
            FnRef(2),                   // 10: false
            FnRef(2),                   // 11: false
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(4),   // true
            FnRef(4),
            Path
        ]);
    }

    #[test]
    fn not_not_false() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // not(bool) -> bool
            FnRef(6),                   // 6: not
            FnRef(0),                   // 7: bool,
            OpRef(1),                   // 8: -> bool
            // not([true] true) -> [false] false
            FnRef(6),                   // 9: not
            Path,
            FnRef(4),                   // 11: true
            FnRef(4),                   // 12: true
            OpRef(5),                   // 13: -> [false] false,
            // not([false] false) -> [true] true
            FnRef(6),                   // 14: not
            Path,
            FnRef(2),                   // 16: false
            FnRef(2),                   // 17: false
            OpRef(9),                   // 18: -> [true] true
        ];

        let ops = vec![
            // bool
            End,
            FnRef(0),                   // 1: bool
            // [false] false
            End,
            Path,
            FnRef(2),                   // 4: false
            FnRef(2),                   // 5: false
            // [true] true
            End,
            Path,
            FnRef(4),                   // 8: false
            FnRef(4),                   // 9: false
            // call not([true] true)
            End,
            Call,
            FnRef(6),                   // 8: not
            Call,
            FnRef(6),                   // 10: not
            Path,
            FnRef(2),                   // 12: false
            FnRef(2),                   // 13: false
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(2),   // false
            FnRef(2),
            Path
        ]);
    }

    #[test]
    fn not_type() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // not(bool) -> bool
            FnRef(6),                   // 6: not
            FnRef(0),                   // 7: bool,
            OpRef(1),                   // 8: -> bool
            // not([true] true) -> [false] false
            FnRef(6),                   // 9: not
            Path,
            FnRef(4),                   // 11: true
            FnRef(4),                   // 12: true
            OpRef(5),                   // 13: -> [false] false,
        ];

        let ops = vec![
            // bool
            End,
            FnRef(0),                   // 1: bool
            // [false] false
            End,
            Path,
            FnRef(2),                   // 4: false
            FnRef(2),                   // 5: false
            // call not(bool)
            End,
            Call,
            FnRef(6),                   // 8: not
            FnRef(0),
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(0),   // bool
        ]);
    }

    #[test]
    fn identity() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // id(X) -> X
            FnRef(6),                   // 6: id
            Var(0),                     // 7: X,
            OpRef(1),                   // 8: -> X
        ];

        let ops = vec![
            // X
            End,
            Var(0),                     // 1: X
            End,
            Call,
            FnRef(6),                   // 8: id
            Path,
            FnRef(2),                   // 10: [false]
            FnRef(2),                   // 11: false
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(2),   // false
            FnRef(2),   // [false]
            Path,
        ]);
    }

    #[test]
    fn equal() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // eq(X, X) -> [true] true
            FnRef(6),                   // 6: eq
            Var(0),                     // 7: X,
            Var(0),                     // 8: X,
            OpRef(3),                   // 9: -> [true] true
        ];

        let ops = vec![
            // [true] true
            End,
            Path,
            FnRef(4),                   // 2: [true]
            FnRef(4),                   // 1: true
            End,
            Call,
            FnRef(6),                   // 8: eq
            FnRef(0),                   // 9: bool
            FnRef(0),                   // 10: bool
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(4),   // true
            FnRef(4),   // [true]
            Path,
        ]);
    }

    #[test]
    fn not_equal() {
        let fns = vec![
            // bool,
            FnRef(0),                   // 0: bool
            End,
            // false,
            FnRef(2),                   // 2: false
            End,
            // true,
            FnRef(4),                   // 4: true
            End,
            // eq(X, X) -> [true] true
            FnRef(6),                   // 6: eq
            Var(0),                     // 7: X,
            Var(0),                     // 8: X,
            OpRef(3),                   // 9: -> [true] true
            // eq(X, Y) -> [false] false
            FnRef(6),                   // 6: eq
            Var(0),                     // 7: X,
            Var(1),                     // 8: Y,
            OpRef(7),                   // 9: -> [false] false
        ];

        let ops = vec![
            // [true] true
            End,
            Path,
            FnRef(4),                   // 2: [true]
            FnRef(4),                   // 3: true
            // [false] false
            End,
            Path,
            FnRef(2),                   // 6: [false]
            FnRef(2),                   // 7: false
            End,
            Call,
            FnRef(6),                   // 8: eq
            FnRef(4),                   // 9: true
            FnRef(2),                   // 10: false
        ];

        let mut stack = vec![];

        eval(&fns, &ops, &mut stack);

        assert_eq!(&stack, &[
            FnRef(2),   // false
            FnRef(2),   // [false]
            Path,
        ]);
    }

    #[test]
    fn test_convert() {
        use piston_meta::parse;

        println!("TEST");
        let rules = ::syntax_rules();
        let source = ::file_to_string("assets/test.txt").unwrap();
        let data = parse(&rules, &source).unwrap();
        let (_fns, _ops) = convert(&data);

    }
}
