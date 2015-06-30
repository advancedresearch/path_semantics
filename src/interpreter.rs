//! An interpreter implementation.

/// Instructions.
#[derive(PartialEq, Debug)]
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
pub fn eval(fns: &[Op], ops: &[Op], st: &mut Vec<Op>, calls: &mut Vec<usize>) {

}

#[cfg(test)]
mod tests {
    use super::*;
    use super::Op::*;

    #[test]
    fn not() {
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
        let mut calls = vec![];

        eval(&fns, &ops, &mut stack, &mut calls);

        assert_eq!(&stack, &[
        ]);
    }
}
