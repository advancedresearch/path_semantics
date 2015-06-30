//! An interpreter implementation.

/// Instructions.
#[derive(PartialEq)]
pub enum Op {
    /// Treats the next instruction as a function call.
    Call,
    /// Treats the next instruction as argument path.
    Path,
    /// Treats the next instruction as function path.
    FnPath,
    /// A function reference.
    FnRef(usize),
    /// A return reference.
    RetRef(usize),
}

/// Evaluates a program.
pub fn eval(fns: &[Op], rets: &[Op], ops: &[Op], st: &mut Vec<Op>) {

}
