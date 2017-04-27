use std::sync::Arc;

/// Stores boolean function.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Expr {
    /// `false` is considered a function with 0 arguments.
    False,
    /// `true` is considered a function with 0 argument.
    True,
    /// All functions are constructed from sub functions.
    If(Arc<Expr>, Arc<Expr>),
}

impl Expr {
    pub fn false_1() -> Expr {
        Expr::If(Arc::new(Expr::False), Arc::new(Expr::False))
    }

    pub fn not() -> Expr {
        Expr::If(Arc::new(Expr::False), Arc::new(Expr::True))
    }

    pub fn id() -> Expr {
        Expr::If(Arc::new(Expr::True), Arc::new(Expr::False))
    }

    pub fn true_1() -> Expr {
        Expr::If(Arc::new(Expr::True), Arc::new(Expr::True))
    }

    pub fn and() -> Expr {
        Expr::If(Arc::new(Expr::id()), Arc::new(Expr::false_1()))
    }

    pub fn or() -> Expr {
        Expr::If(Arc::new(Expr::true_1()), Arc::new(Expr::id()))
    }

    pub fn false_n(n: usize) -> Expr {
        if n == 0 {
            Expr::False
        } else {
            let sub = Arc::new(Expr::false_n(n - 1));
            Expr::If(sub.clone(), sub)
        }
    }

    pub fn true_n(n: usize) -> Expr {
        if n == 0 {
            Expr::True
        } else {
            let sub = Arc::new(Expr::true_n(n - 1));
            Expr::If(sub.clone(), sub)
        }
    }

    /// Creates the symmetric path by `not`.
    pub fn sympath_not(&self) -> Expr {
        match *self {
            Expr::False => Expr::True,
            Expr::True => Expr::False,
            Expr::If(ref a, ref b) => {
                if a == b {
                    let sub = Arc::new(a.sympath_not());
                    Expr::If(sub.clone(), sub)
                } else {
                    Expr::If(Arc::new(b.sympath_not()), Arc::new(a.sympath_not()))
                }
            }
        }
    }

    /// Returns `true` if argument is irrelevant.
    ///
    /// This is the case when the two sub expressions are equal.
    pub fn is_irrelevant(&self) -> bool {
        if let Expr::If(ref a, ref b) = *self {
            a == b
        } else {
            false
        }
    }

    /// Evaluates the function with one argument.
    pub fn apply(&self, val: bool) -> Option<&Expr> {
        if let Expr::If(ref a, ref b) = *self {
            Some(if val {a} else {b})
        } else {
            None
        }
    }

    pub fn collapses(&self) -> bool {
        self == &Expr::false_1() || self == &Expr::true_1()
    }

    pub fn preserves_structure(&self) -> bool {
        self == &Expr::not() || self == &Expr::id()
    }

    pub fn has_asympath(&self, ps: &[Expr], tension: bool) -> bool {
        // When both sub functions are the same,
        // the argument is irrelevant, so it does not matter
        // what the path does.
        if self.is_irrelevant() {
            if let Expr::If(ref a, _) = *self {
                // The tension is inherited.
                return a.has_asympath(&ps[1..], tension)
            } else {
                unreachable!()
            }
        }

        if ps[0].collapses() {
            // Collapse argument.
            match *self {
                // Last value, tension is resolved by collapsing.
                Expr::False | Expr::True => true,
                Expr::If(ref a, ref b) => {
                    a.has_asympath(&ps[1..], true) &&
                    b.has_asympath(&ps[1..], true)
                }
            }
        } else if ps[0].preserves_structure() {
            // Preserve structure.
            match *self {
                // Last value, path exists if there is no tension.
                Expr::False | Expr::True => !tension,
                Expr::If(ref a, ref b) => {
                    a.has_asympath(&ps[1..], tension) &&
                    b.has_asympath(&ps[1..], tension)
                }
            }
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_irrelevant() {
        assert!(Expr::false_1().is_irrelevant());
        assert!(!Expr::not().is_irrelevant());
        assert!(!Expr::id().is_irrelevant());
        assert!(Expr::true_1().is_irrelevant());
    }

    #[test]
    fn test_sympath_not() {
        assert_eq!(Expr::and().sympath_not(), Expr::or());
        assert_eq!(Expr::or().sympath_not(), Expr::and());
    }

    #[test]
    fn test_has_asympath() {
        // false[false_1] <=> false
        assert!(Expr::False.has_asympath(&[Expr::false_1()], false));
        // false[not] <=> true
        assert!(Expr::False.has_asympath(&[Expr::not()], false));
        // false[id] <=> false
        assert!(Expr::False.has_asympath(&[Expr::id()], false));
        // false[true_1] <=> true
        assert!(Expr::False.has_asympath(&[Expr::true_1()], false));

        // true[false_1] <=> false
        assert!(Expr::True.has_asympath(&[Expr::false_1()], false));
        // true[not] <=> false
        assert!(Expr::True.has_asympath(&[Expr::not()], false));
        // true[id] <=> true
        assert!(Expr::True.has_asympath(&[Expr::id()], false));
        // true[true_1] <=> true
        assert!(Expr::True.has_asympath(&[Expr::true_1()], false));

        // false_1[false_1 -> false_1] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // false_1[false_1 -> not] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::false_1(),
                Expr::not()
            ], false));
        // false_1[false_1 -> id] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::false_1(),
                Expr::id()
            ], false));
        // false_1[false_1 -> true_1] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // false_1[not -> false_1] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::not(),
                Expr::false_1()
            ], false));
        // false_1[not -> not] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::not(),
                Expr::not()
            ], false));
        // false_1[not -> id] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::not(),
                Expr::id()
            ], false));
        // false_1[not -> true_1] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::not(),
                Expr::true_1()
            ], false));

        // false_1[id -> false_1] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::id(),
                Expr::false_1()
            ], false));
        // false_1[id -> not] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::id(),
                Expr::not()
            ], false));
        // false_1[id -> id] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::id(),
                Expr::id()
            ], false));
        // false_1[id -> true_1] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::id(),
                Expr::true_1()
            ], false));

        // false_1[true_1 -> false_1] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // false_1[true_1 -> not] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::true_1(),
                Expr::not()
            ], false));
        // false_1[true_1 -> id] <=> false_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::true_1(),
                Expr::id()
            ], false));
        // false_1[true_1 -> true_1] <=> true_1
        assert!(Expr::false_1().has_asympath(&[
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // not[false_1 -> false_1] <=> false_1
        assert!(Expr::not().has_asympath(&[
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // not[false_1 -> not] <=> {}
        assert!(!Expr::not().has_asympath(&[
                Expr::false_1(),
                Expr::not()
            ], false));
        // not[false_1 -> id] <=> {}
        assert!(!Expr::not().has_asympath(&[
                Expr::false_1(),
                Expr::id()
            ], false));
        // not[false_1 -> true_1] <=> true_1
        assert!(Expr::not().has_asympath(&[
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // not[not -> false_1] <=> false_1
        assert!(Expr::not().has_asympath(&[
                Expr::not(),
                Expr::false_1()
            ], false));
        // not[not -> not] <=> not
        assert!(Expr::not().has_asympath(&[
                Expr::not(),
                Expr::not()
            ], false));
        // not[not -> id] <=> id
        assert!(Expr::not().has_asympath(&[
                Expr::not(),
                Expr::id()
            ], false));
        // not[not -> true_1] <=> true_1
        assert!(Expr::not().has_asympath(&[
                Expr::not(),
                Expr::true_1()
            ], false));

        // not[id -> false_1] <=> false_1
        assert!(Expr::not().has_asympath(&[
                Expr::id(),
                Expr::false_1()
            ], false));
        // not[id -> not] <=> id
        assert!(Expr::not().has_asympath(&[
                Expr::id(),
                Expr::not()
            ], false));
        // not[id -> id] <=> not
        assert!(Expr::not().has_asympath(&[
                Expr::id(),
                Expr::id()
            ], false));
        // not[id -> true_1] <=> true_1
        assert!(Expr::not().has_asympath(&[
                Expr::id(),
                Expr::true_1()
            ], false));

        // not[true_1 -> false_1] <=> false_1
        assert!(Expr::not().has_asympath(&[
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // not[true_1 -> not] <=> {}
        assert!(!Expr::not().has_asympath(&[
                Expr::true_1(),
                Expr::not()
            ], false));
        // not[true_1 -> id] <=> {}
        assert!(!Expr::not().has_asympath(&[
                Expr::true_1(),
                Expr::id()
            ], false));
        // not[true_1 -> true_1] <=> true_1
        assert!(Expr::not().has_asympath(&[
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // id[false_1 -> false_1] <=> false_1
        assert!(Expr::id().has_asympath(&[
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // id[false_1 -> not] <=> {}
        assert!(!Expr::id().has_asympath(&[
                Expr::false_1(),
                Expr::not()
            ], false));
        // id[false_1 -> id] <=> {}
        assert!(!Expr::id().has_asympath(&[
                Expr::false_1(),
                Expr::id()
            ], false));
        // id[false_1 -> true_1] <=> true_1
        assert!(Expr::id().has_asympath(&[
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // id[not -> false_1] <=> false_1
        assert!(Expr::id().has_asympath(&[
                Expr::not(),
                Expr::false_1()
            ], false));
        // id[not -> not] <=> id
        assert!(Expr::id().has_asympath(&[
                Expr::not(),
                Expr::not()
            ], false));
        // id[not -> id] <=> not
        assert!(Expr::id().has_asympath(&[
                Expr::not(),
                Expr::id()
            ], false));
        // id[not -> true_1] <=> true_1
        assert!(Expr::id().has_asympath(&[
                Expr::not(),
                Expr::true_1()
            ], false));

        // id[id -> false_1] <=> false_1
        assert!(Expr::id().has_asympath(&[
                Expr::id(),
                Expr::false_1()
            ], false));
        // id[id -> not] <=> not
        assert!(Expr::id().has_asympath(&[
                Expr::id(),
                Expr::not()
            ], false));
        // id[id -> id] <=> id
        assert!(Expr::id().has_asympath(&[
                Expr::id(),
                Expr::id()
            ], false));
        // id[id -> true_1] <=> true_1
        assert!(Expr::id().has_asympath(&[
                Expr::id(),
                Expr::true_1()
            ], false));

        // id[true_1 -> false_1] <=> false_1
        assert!(Expr::id().has_asympath(&[
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // id[true_1 -> not] <=> {}
        assert!(!Expr::id().has_asympath(&[
                Expr::true_1(),
                Expr::not()
            ], false));
        // id[true_1 -> id] <=> {}
        assert!(!Expr::id().has_asympath(&[
                Expr::true_1(),
                Expr::id()
            ], false));
        // id[true_1 -> true_1] <=> true_1
        assert!(Expr::id().has_asympath(&[
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // true_1[false_1 -> false_1] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // true_1[false_1 -> not] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::false_1(),
                Expr::not()
            ], false));
        // true_1[false_1 -> id] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::false_1(),
                Expr::id()
            ], false));
        // true_1[false_1 -> true_1] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // true_1[not -> false_1] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::not(),
                Expr::false_1()
            ], false));
        // true_1[not -> not] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::not(),
                Expr::not()
            ], false));
        // true_1[not -> id] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::not(),
                Expr::id()
            ], false));
        // true_1[not -> true_1] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::not(),
                Expr::true_1()
            ], false));

        // true_1[id -> false_1] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::id(),
                Expr::false_1()
            ], false));
        // true_1[id -> not] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::id(),
                Expr::not()
            ], false));
        // true_1[id -> id] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::id(),
                Expr::id()
            ], false));
        // true_1[id -> true_1] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::id(),
                Expr::true_1()
            ], false));

        // true_1[true_1 -> false_1] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // true_1[true_1 -> not] <=> false_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::true_1(),
                Expr::not()
            ], false));
        // true_1[true_1 -> id] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::true_1(),
                Expr::id()
            ], false));
        // true_1[true_1 -> true_1] <=> true_1
        assert!(Expr::true_1().has_asympath(&[
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // and[false_1 -> false_1 -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // and[false_1 -> false_1 -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::false_1(),
                Expr::not()
            ], false));
        // and[false_1 -> false_1 -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::false_1(),
                Expr::not()
            ], false));
        // and[false_1 -> false_1 -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // and[false_1 -> not -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::not(),
                Expr::false_1()
            ], false));
        // and[false_1 -> not -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::not(),
                Expr::not()
            ], false));
        // and[false_1 -> not -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::not(),
                Expr::id()
            ], false));
        // and[false_1 -> not -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::not(),
                Expr::true_1()
            ], false));

        // and[false_1 -> id -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::id(),
                Expr::false_1()
            ], false));
        // and[false_1 -> id -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::id(),
                Expr::not()
            ], false));
        // and[false_1 -> id -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::id(),
                Expr::id()
            ], false));
        // and[false_1 -> id -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::id(),
                Expr::true_1()
            ], false));

        // and[false_1 -> true_1 -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // and[false_1 -> true_1 -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::true_1(),
                Expr::not()
            ], false));
        // and[false_1 -> true_1 -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::true_1(),
                Expr::id()
            ], false));
        // and[false_1 -> true_1 -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::false_1(),
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // and[not -> false_1 -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::false_1(),
                Expr::false_1()
            ], false));
        // and[not -> false_1 -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::not(),
                Expr::false_1(),
                Expr::not()
            ], false));
        // and[not -> false_1 -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::not(),
                Expr::false_1(),
                Expr::id()
            ], false));
        // and[not -> false_1 -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::false_1(),
                Expr::true_1()
            ], false));

        // and[not -> not -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::not(),
                Expr::false_1()
            ], false));
        // and[not -> not -> not] <=> or
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::not(),
                Expr::not()
            ], false));
        // and[not -> not -> id] <=> nor
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::not(),
                Expr::id()
            ], false));
        // and[not -> not -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::not(),
                Expr::true_1()
            ], false));

        // and[not -> id -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::id(),
                Expr::false_1()
            ], false));
        // and[not -> id -> not] <=> ?
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::id(),
                Expr::not()
            ], false));
        // and[not -> id -> id] <=> ?
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::id(),
                Expr::id()
            ], false));
        // and[not -> id -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::id(),
                Expr::true_1()
            ], false));

        // and[not -> true_1 -> false_1] <=> false_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::true_1(),
                Expr::false_1()
            ], false));
        // and[not -> true_1 -> not] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::not(),
                Expr::true_1(),
                Expr::not()
            ], false));
        // and[not -> true_1 -> id] <=> {}
        assert!(!Expr::and().has_asympath(&[
                Expr::not(),
                Expr::true_1(),
                Expr::id()
            ], false));
        // and[not -> true_1 -> true_1] <=> true_2
        assert!(Expr::and().has_asympath(&[
                Expr::not(),
                Expr::true_1(),
                Expr::true_1()
            ], false));

        // and[and -> id] <=> id
    }
}
