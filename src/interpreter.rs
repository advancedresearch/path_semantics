//! An interpreter implementation.

use piston_meta::*;
use range::Range;
use std::sync::Arc;

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
pub fn convert(
    mut data: &[Range<MetaData>],
    ignored: &mut Vec<Range>
) -> Result<(Vec<Op>, Vec<Op>), ()> {
    use piston_meta::bootstrap::{ end_node, ignore, start_node, update,
        meta_string };

    // Stores the state of function and instruction state.
    #[derive(Copy, Clone)]
    struct ConvertState(usize, usize);

    fn push_fn(c: &ConvertState, fns: &mut Vec<Op>, op: Op) -> ConvertState {
        if c.0 < fns.len() {
            fns.truncate(c.0);
        }
        fns.push(op);
        ConvertState(c.0 + 1, c.1)
    }

    fn push_op(c: &ConvertState, ops: &mut Vec<Op>, op: Op) -> ConvertState {
        if c.1 < ops.len() {
            ops.truncate(c.1);
        }
        ops.push(op);
        ConvertState(c.0, c.1 + 1)
    }

    fn find_name(
        name: Arc<String>,
        names: &[(Arc<String>, usize)]
    ) -> Option<usize> {
        names.iter().find(|e| e.0 == name).map(|e| e.1)
    }

    fn read_ret(
        mut data: &[Range<MetaData>],
        mut offset: usize,
        _fns: &mut Vec<Op>,
        ops: &mut Vec<Op>,
        state: &ConvertState,
        names: &mut Vec<(Arc<String>, usize)>,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, ConvertState), ()> {
        let mut new_state = state.clone();
        let start_offset = offset;
        let node = "ret";
        let range = try!(start_node(node, data, offset));
        let mut new_ops = vec![];
        update(range, &mut data, &mut offset);
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("path", data, offset) {
                update(range, &mut data, &mut offset);
                let index = match find_name(val, names) {
                    None => { return Err(()); }
                    Some(index) => index,
                };
                new_ops.push(Op::Path);
                new_ops.push(Op::FnRef(index));
            } else if let Ok((range, val)) = meta_string("ns_name", data, offset) {
                update(range, &mut data, &mut offset);
                let index = match find_name(val, names) {
                    None => { return Err(()); }
                    Some(index) => index,
                };
                new_ops.push(Op::FnRef(index));
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        new_ops.push(Op::End);

        // Reverse instructions.
        for op in new_ops.into_iter().rev() {
            new_state = push_op(&new_state, ops, op);
        }
        Ok((Range::new(start_offset, offset - start_offset), new_state))
    }

    fn read_arg(
        mut data: &[Range<MetaData>],
        mut offset: usize,
        fns: &mut Vec<Op>,
        _ops: &mut Vec<Op>,
        state: &ConvertState,
        names: &mut Vec<(Arc<String>, usize)>,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, ConvertState), ()> {
        let mut new_state = state.clone();
        let start_offset = offset;
        let node = "arg";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, val)) = meta_string("path", data, offset) {
                update(range, &mut data, &mut offset);
                let index = match find_name(val, names) {
                    None => { return Err(()); }
                    Some(index) => index,
                };
                new_state = push_fn(&new_state, fns, Op::Path);
                new_state = push_fn(&new_state, fns, Op::FnRef(index));
            } else if let Ok((range, val)) = meta_string("ns_name", data, offset) {
                update(range, &mut data, &mut offset);
                let index = match find_name(val, names) {
                    None => { return Err(()); }
                    Some(index) => index,
                };
                new_state = push_fn(&new_state, fns, Op::FnRef(index));
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        Ok((Range::new(start_offset, offset - start_offset), new_state))
    }

    fn read_fn(
        mut data: &[Range<MetaData>],
        mut offset: usize,
        fns: &mut Vec<Op>,
        ops: &mut Vec<Op>,
        state: &ConvertState,
        names: &mut Vec<(Arc<String>, usize)>,
        ignored: &mut Vec<Range>
    ) -> Result<(Range, ConvertState), ()> {
        let mut new_state = state.clone();
        let start_offset = offset;
        let node = "fn";
        let range = try!(start_node(node, data, offset));
        update(range, &mut data, &mut offset);
        loop {
            if let Ok(range) = end_node(node, data, offset) {
                update(range, &mut data, &mut offset);
                break;
            } else if let Ok((range, name)) = meta_string("name", data, offset) {
                update(range, &mut data, &mut offset);
                let index = match find_name(name.clone(), names) {
                    None => { fns.len() }
                    Some(index) => index,
                };
                names.push((name, index));
                new_state = push_fn(&new_state, fns, Op::FnRef(index));
            } else if let Ok((range, state)) = read_arg(
                    data, offset, fns, ops, &new_state, names, ignored
                ) {
                update(range, &mut data, &mut offset);
                new_state = state;
            } else if let Ok((range, state)) = read_ret(
                    data, offset, fns, ops, &new_state, names, ignored
                ) {
                update(range, &mut data, &mut offset);
                new_state = push_fn(&state, fns, Op::OpRef(ops.len() - 1));
            } else {
                let range = ignore(data, offset);
                update(range, &mut data, &mut offset);
                ignored.push(range);
            }
        }

        new_state = push_fn(&new_state, fns, Op::End);
        Ok((Range::new(start_offset, offset - start_offset), new_state))
    }

    let mut fns = vec![];
    let mut ops = vec![];
    let mut names = vec![];
    let mut offset = 0;
    let mut state = ConvertState(0, 0);

    loop {
        if let Ok((range, new_state)) = read_fn(data, offset, &mut fns,
            &mut ops, &state, &mut names, ignored)
        {
            update(range, &mut data, &mut offset);
            state = new_state;
        } else if offset < data.len() {
            return Err(());
        } else {
            break;
        }
    }

    Ok((fns, ops))
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
    fn convert_bool() {
        use piston_meta::parse;

        let rules = ::syntax_rules();
        let source = "
fn bool() -> bool;
fn true(bool) -> true;
fn false(bool) -> false;
        ";
        let mut data = vec![];
        parse(&rules, &source, &mut data).unwrap();
        let (fns, ops) = convert(&data, &mut vec![]).unwrap();
        assert_eq!(&fns, &[
            // bool() -> bool
            FnRef(0),           // bool
            OpRef(1),           // -> bool
            End,
            // true(bool) -> true
            FnRef(3),           // true
            FnRef(0),           // bool
            OpRef(3),           // -> true
            End,
            // false(bool) -> false
            FnRef(7),           // false
            FnRef(0),           // bool
            OpRef(5),           // -> false
            End
        ]);
        assert_eq!(&ops, &[
            End,
            FnRef(0),           // bool
            End,
            FnRef(3),           // true
            End,
            FnRef(7)            // false
        ]);
    }

    #[test]
    fn convert_path() {
        use piston_meta::*;

        let rules = ::syntax_rules();
        let source = "
fn bool() -> bool;
fn true(bool) -> true;
fn false(bool) -> false;
fn or(bool, bool) -> bool;
fn or([false] false, [false] false) -> [false] false;
        ";
        let mut data = vec![];
        parse(&rules, &source, &mut data).unwrap();
        // json::print(&data);
        let (fns, ops) = convert(&data, &mut vec![]).unwrap();
        assert_eq!(&fns, &vec![
            // bool() -> bool
            FnRef(0),
            OpRef(1),
            End,
            // true(bool) -> true
            FnRef(3),
            FnRef(0),
            OpRef(3),
            End,
            // false(bool) -> false
            FnRef(7),
            FnRef(0),
            OpRef(5),
            End,
            // or(bool, bool) -> bool
            FnRef(11),
            FnRef(0),
            FnRef(0),
            OpRef(7),
            End,
            // or([false] false, [false] false) -> [false] false
            FnRef(11),
            Path,
            FnRef(7),
            FnRef(7),
            Path,
            FnRef(7),
            FnRef(7),
            OpRef(11),
            End]);
        assert_eq!(&ops, &[
            // -> bool
            End,
            FnRef(0),
            // -> true
            End,
            FnRef(3),
            // -> false
            End,
            FnRef(7),
            // -> bool
            End,
            FnRef(0),
            // -> [false] false
            End,
            FnRef(7),
            FnRef(7),
            Path
        ]);
    }
}
