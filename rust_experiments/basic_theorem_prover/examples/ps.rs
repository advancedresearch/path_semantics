#![feature(box_patterns)]

extern crate monotonic_solver;

use monotonic_solver::*;
use std::collections::HashSet;
use std::fmt::{self, Display, Formatter};

use Expression::*;

#[derive(Clone, Hash, PartialEq, Eq, Debug)]
pub enum Expression {
    False,
    True,
    If(Box<Expression>, Box<Expression>),
    Path(Box<Expression>, Box<Expression>),
    Variable(&'static str),
    IsType(Box<Expression>, Box<Expression>),
    Define(Box<Expression>, Box<Expression>),
    Apply(Box<Expression>, Box<Expression>),
    Check(Box<Expression>, Box<Expression>),
    Eq(Box<Expression>, Box<Expression>),
    Ex(Box<Expression>),
}

impl Display for Expression {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match *self {
            False => write!(fmt, "false")?,
            True => write!(fmt, "true")?,
            If(ref a, ref b) => write!(fmt, "if({}, {})", a, b)?,
            Path(ref a, ref b) => write!(fmt, "[{}] {}", a, b)?,
            Variable(a) => write!(fmt, "{}", a)?,
            IsType(ref a, ref b) => write!(fmt, "{} : {}", a, b)?,
            Define(ref a, ref b) => write!(fmt, "{} := {}", a, b)?,
            Apply(ref a, ref b) => write!(fmt, "{}({})", a, b)?,
            Check(ref a, ref b) => write!(fmt, "check `{}` is `{}`", a, b)?,
            Eq(ref a, ref b) => write!(fmt, "{} == {}", a, b)?,
            Ex(ref a) => write!(fmt, "(∃{})", a)?,
        }
        Ok(())
    }
}

fn infer(
    cache: &HashSet<Expression>,
    filter_cache: &HashSet<Expression>,
    story: &[Expression]
) -> Option<Expression> {
    let can_add = |new_expr: &Expression| {
        !cache.contains(new_expr) &&
        !filter_cache.contains(new_expr)
    };

    for expr in story {
        if let &Define(ref l, ref r) = expr {
            let new_expr = Define(l.clone(), eval(r, story));
            if can_add(&new_expr) {return Some(new_expr);}
        }

        if let &IsType(ref l, box Path(ref f, ref v)) = expr {
            for expr2 in story {
                if let &Define(ref l2, ref val) = expr2 {
                    if l2 == l {
                        let new_expr = Check(Box::new(expr.clone()),
                                eval(&eq(apply(f.clone(), val.clone()), v.clone()), story)
                            );
                        if can_add(&new_expr) {return Some(new_expr);}
                    }
                }
            }

            let new_expr = Check(is_type(v.clone(), path(ex(f.clone()), true_())),
                    eval(&eq(apply(ex(f.clone()), v.clone()), true_()), story)
                );
            if can_add(&new_expr) {return Some(new_expr);}
        }
    }

    None
}

fn eval(expr: &Expression, exprs: &[Expression]) -> Box<Expression> {
    match expr {
        &False => Box::new(False),
        &True => Box::new(True),
        &Apply(box If(ref t, ref f), box If(ref vt, ref vf)) => {
            eval(&apply(apply(if_(t.clone(), f.clone()), vt.clone()), vf.clone()), exprs)
        }
        &Apply(box If(ref t, ref f), ref v) => {
            match **v {
                False => eval(f, exprs),
                True => eval(t, exprs),
                _ => {
                    let new_v = eval(v, exprs);
                    match *new_v {
                        False => eval(f, exprs),
                        True => eval(t, exprs),
                        _ => Box::new(expr.clone())
                    }
                }
            }
        }
        &Apply(box Ex(box If(ref t, ref f)), ref v) => {
            let new_t = eval(t, exprs);
            let new_f = eval(f, exprs);
            let new_v = eval(v, exprs);
            if new_v == new_t || new_v == new_f {
                true_()
            } else {
                if *eval(&apply(ex(new_t), new_v.clone()), exprs) == True ||
                   *eval(&apply(ex(new_f), new_v), exprs) == True {
                    true_()
                } else {
                    false_()
                }
            }
        }
        &Apply(ref f, ref v) => {
            let new_f = eval(f, exprs);
            let new_v = eval(v, exprs);
            if f != &new_f || v != &new_v {
                eval(&apply(new_f, new_v), exprs)
            } else {
                Box::new(expr.clone())
            }
        }
        &Variable(a) => {
            for expr in exprs {
                if let &Define(box Variable(b), ref val) = expr {
                    if a == b {
                        return val.clone()
                    }
                }
            }
            Box::new(expr.clone())
        }
        &If(ref t, ref f) => {
            let new_t = eval(t, exprs);
            let new_f = eval(f, exprs);
            if t != &new_t || f != &new_f {
                Box::new(If(new_t, new_f))
            } else {
                Box::new(expr.clone())
            }
        }
        &Eq(ref l, ref r) => {
            let new_l = eval(l, exprs);
            let new_r = eval(r, exprs);
            if new_l == new_r {
                Box::new(True)
            } else if let (&box False, &box True) = (&new_l, &new_r) {
                Box::new(False)
            } else if let (&box True, &box False) = (&new_l, &new_r) {
                Box::new(False)
            } else {
                eq(new_l, new_r)
            }
        }
        &Ex(ref f) => {
            let new_f = eval(f, exprs);
            if f != &new_f {
                eval(&Ex(new_f), exprs)
            } else {
                Box::new(expr.clone())
            }
        }
        x => unimplemented!("{:?}", x),
    }
}

fn false_() -> Box<Expression> {Box::new(False)}
fn true_() -> Box<Expression> {Box::new(True)}
fn if_(a: Box<Expression>, b: Box<Expression>) -> Box<Expression> {
    Box::new(If(a, b))
}
fn var(name: &'static str) -> Box<Expression> {Box::new(Variable(name))}
fn apply(f: Box<Expression>, value: Box<Expression>) -> Box<Expression> {
    Box::new(Apply(f, value))
}
fn path(f: Box<Expression>, value: Box<Expression>) -> Box<Expression> {
    Box::new(Path(f, value))
}
fn is_type(a: Box<Expression>, ty: Box<Expression>) -> Box<Expression> {
    Box::new(IsType(a, ty))
}
fn eq(a: Box<Expression>, b: Box<Expression>) -> Box<Expression> {
    Box::new(Eq(a, b))
}
fn ex(a: Box<Expression>) -> Box<Expression> {
    Box::new(Ex(a))
}

/*
a : [false_1] true
a : [false_1] false

a : [false_1{[false_1] true}] false
a : [false_1{[false_1] false}] true

(false, false)

If you use lambdas as tuples, then you can access the first element by
applying it to `true` and the second element by applying it to `false`.
So, when evaluating a function and you pass it another function,
then it should use the first element as the first bit and use the
second element, a tail, as arguments for the rest.

When you check the following, it seems to operate on two different types:

a : [or] true
b : [or] id

true : [∃or] true
id : [∃or] true

One idea is to create a custom evaluation that checks for equality
with each branch. If there is no equality then it takes the
existential path of each branch and repeats the procedure.
When you get to single argument functions,
you do hard coded checks.

This is a new apply rule but for existential paths.

The double-existential path can be found by creating a tuple
of the existential path applied to `true` and `false`.

∃∃f <=> if((∃f)(true), (∃f)(false))
    if(false, false)
    if(false, true)
    if(true, false)
    if(true, true)

*/

fn main() {
    let start = vec![
        Define(var("false_"), if_(false_(), false_())),
        Define(var("not"), if_(false_(), true_())),
        Define(var("id"), if_(true_(), false_())),
        Define(var("true_1"), if_(true_(), true_())),
        Define(var("and"), if_(var("id"), var("false_1"))),
        Define(var("or"), if_(var("true_1"), var("id"))),

        Define(var("a"), true_()),
        IsType(var("a"), path(var("and"), var("id"))),
        IsType(var("b"), path(var("and"), true_())),
        IsType(var("c"), path(var("true_1"), true_())),
    ];
    let goal = vec![
        Check(is_type(var("a"), path(var("and"), var("id"))), true_()),
        Check(is_type(var("id"), path(ex(var("and")), true_())), true_()),
        Check(is_type(true_(), path(ex(var("and")), true_())), true_()),
        Check(is_type(true_(), path(ex(var("true_1")), true_())), true_()),
    ];
    let filter = vec![];
    let after_constraints = vec![];
    match solve_and_reduce(&start, &goal, &filter, &after_constraints, infer) {
        Ok(solution) => {
            for expr in &solution {
                println!("{}", expr);
            }
        }
        Err(solution) => {
            for expr in &solution {
                println!("{}", expr);
            }
            println!("Could not solve");
        }
    }
}
