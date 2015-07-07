//! An interpreter implementation.

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
    /// End of instructions.
    End,
}

/// Evaluates a program.
pub fn eval(fns: &[Op], ops: &[Op], st: &mut Vec<Op>) {
    let mut calls: Vec<usize> = Vec::new();
    let mut call = ops.len();
    'e: loop {
        let mut f = None;
        // Push instructions.
        for op in ops[0..call].iter().rev() {
            match op {
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
            // Pattern match function.
            'f: loop {
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
                    j -= 1;
                    if fi != &st[j] {
                        if i == 0 {
                            panic!("No function matched (failed to match name)");
                        }
                        // Try next function.
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
}
