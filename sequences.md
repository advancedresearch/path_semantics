# Sequences for learning Path Semantics
Reading sequences for learning path semantics.

This is a collection of papers organized in a way to build up gradually understanding of path semantics.  
Some papers are marked with stars (e.g. ★★★★★) to signify their significance.

### Cheat Sheets

- [Logi](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/logi.pdf) ★★★★★
- [Path Semantics Cheat Sheet](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics-cheat-sheet.pdf)
- [Boolean Algebra Cheat Sheet](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/boolean-algebra-cheat-sheet.pdf)

### Reference material

- [Alphabetic List of Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/alphabetic-list-of-functions.pdf)
- [Alphabetic List of Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/alphabetic-list-of-paths.pdf)
- [Alphabetic List of Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/alphabetic-list-of-existential-paths.pdf)
- [Existential Paths of Real Addition on Intervals](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths-of-real-addition-on-intervals.pdf)
- [Mini Toolkit in Dyon for Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/mini-toolkit-in-dyon-for-boolean-path-semantics.pdf)
- [Terminology for Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/terminology-for-binary-relations.pdf)
- [Terminology for Morphisms](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/terminology-for-morphisms.pdf)
- [Semantics of Propositions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/semantics-of-propositions.pdf)

### Background

- [Visualizing Path Semantics Using LEGO Bricks](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/visualizing-path-semantics-using-lego-bricks.pdf)
- [History of Path Semantics (Illustrated With Stick Figures)](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/history-of-path-semantics-illustrated.pdf)
- [How to Think Abstractly With Path Semantics (Illustrated)](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/how-to-think-abstractly-with-path-semantics.pdf)
- [Goals of Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/goals-of-path-semantics.pdf)

### Foundation

Normal paths are formalized in 5 different ways:

- Equation (logical relation between functions)
- Homotopy path interpretation (from the definition of homotopy paths)
- Basic logical primitives (core axiom of Path Semantics)
- [Indirect Proof of Normal Path Composition in Cubical Type Theory](https://github.com/advancedresearch/proof_of_normal_path_composition)
- [Direct Proof of Total Normal Path Composition in Lean 3](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/semiconjugates-as-satisfied-models-of-total-normal-paths.pdf)

The proof in Lean 3 assumes satisfied models of total normal paths using semiconjugates.
This is currently the best available model of normal paths in dependent types.

Each of these pictures can be used to reason about normal paths.
However, there exists no exhausting set of axioms (this is currently believed to be an undecidable problem).
For some theories, like Boolean functions, normal paths are proven to be sound by exhaustive proof search.

In the broader theory of logic, one can study [Path Semantical Logic](#path-semantical-logic).

- [Formal Definition of Normal Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/formal-definition-of-normal-paths.pdf) ★★★★★
- [Atomic Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/atomic-functions.pdf) ★★★★★
- [Symbolic Distinction](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/symbolic-distinction.pdf) ★★★★★
- [Path Semantical Quality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/path-semantical-quality.pdf) ★★★★★
- [Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf) ★★★★★
- [Philosophy of The Core Axiom](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/philosophy-of-the-core-axiom.pdf)
- [Lifted Associations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/lifted-associations.pdf)
- [Asserting Formal Associations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/asserting-formal-associations.pdf)
- [Computational Equivalence](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/computational-equivalence.pdf)
- [Incompleteness of Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/incompleteness-of-path-semantics.pdf)
- [Undefined Symbols](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/undefined-symbols.pdf)
- [Homotopy Level Zero of Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/homotopy-level-zero-of-sub-types.pdf)
- [Category Realizable Groupoids](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/category-realizable-groupoids.pdf)
- [Semiconjugates as Satisfied Models of Total Normal Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/semiconjugates-as-satisfied-models-of-total-normal-paths.pdf) ★★★★★
- [Rewriting and the Core Axiom](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/rewriting-and-the-core-axiom.pdf)

### Path Semantical Logic

*It is recommended to skip this section, unless you are interested in logic or the foundation of Path Semantics.*

Path Semantical Logic uses the core axiom to group propositions into levels.
A quality at level N propagates into quality at level N+1 (2021 standard order).

For an experimental implementations, see:

- [Pocket-Prover](https://github.com/advancedresearch/pocket_prover) (Classical Propositional Logic - PL/PSL)
- [Prop](https://github.com/advancedresearch/prop) (Intuitionistic Propositional Logic - IPL/PSI)
- [Path Semantical Intuitionistic Propositional Logic in Avalog](https://github.com/advancedresearch/avalog/blob/master/source/psi.txt) (Avatar Logic - PSI)

Notice that Avatar Logic is required to weaken IPL to PSI properly.

Some theorems, such as the Implication Theorem or Creation Theorem, holds in PSL but are not provable in PSI without LEM (Law of Excluded Middle).
Since e.g. the Creation Theorem is absurd, this implies that PSI is the natural foundation of Path Semantics.

Papers:

- [New Standard Order for Levels](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/new-standard-order-for-levels.pdf) ★★★★★
- [Witness in Path Semantical Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/witness-in-path-semantical-logic.pdf)
- [Faster Brute Force Proofs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/faster-brute-force-proofs.pdf)
- [Complexity of Path Semantical Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/complexity-of-path-semantical-logic.pdf)
- [Index Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/index-theorem.pdf)
- [Proving Isomorphism Using Indexing](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proving-isomorphism-using-indexing.pdf)
- [Implication Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/implication-theorem.pdf)
- [Abstract Implication Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/abstract-implication-theorem.pdf)
- [Constrained Implication Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-implication-theorem.pdf)
- [Negative Association Error](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/negative-association-error.pdf)
- [Visualizing Implication Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/visualizing-implication-theorems.pdf)
- [Space Jumping Example](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/space-jumping-example.pdf)
- [Twin Implication Core Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/twin-implication-core-theorem.pdf)
- [Concrete and Abstract Transport](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/concrete-and-abstract-transport.pdf)
- [Abstract Transport XOR Trick](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/abstract-transport-xor-trick.pdf)
- [Non-Composition of XOR Trick](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/non-composition-of-xor-trick.pdf)
- [Entangled XOR Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/entangled-xor-theorem.pdf)
- [Exclusive Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/exclusive-theorem.pdf)
- [Negative Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/negative-types.pdf)
- [Noncontractible Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/noncontractible-types.pdf)
- [Contractible Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/contractible-types.pdf)
- [Contraction Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/contraction-theorem.pdf)
- [Collapse Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/collapse-theorems.pdf)
- [Inverse Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/inverse-theorems.pdf)
- [Inner and Outer Transport Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/inner-and-outer-transport-theorems.pdf)
- [Implicit Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/implicit-theorems.pdf)
- [Modeling Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/modeling-functions.pdf) ★★★★★
- [Cubical Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cubical-types.pdf)
- [Duality in Path Semantical Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/duality-in-path-semantical-logic.pdf)
- [Path Function Existence](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-function-existence.pdf)
- [Time Interpretation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/time-interpretation.pdf) ★★★★★
- [Grounding of Causality to The Present](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/grounding-of-causality-to-the-present.pdf)
- [Natural Propositions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/natural-propositions.pdf)
- [Transitive Mirror Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/transitive-mirror-theorems.pdf)
- [Implicit Activation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/implicit-activation.pdf)
- [Creation Theorem](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/creation-theorem.pdf)

### Introduction

- [Introduction to Path Semantics for Computer Scientists](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/introduction-to-path-semantics-for-computer-scientists.pdf)
- [Tutorial](https://github.com/advancedresearch/path_semantics/wiki/Tutorial-1:-Types)  
- [Normal Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/normal-paths.pdf) ★★★★★
- [Sub-Types as Contextual Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/sub-types-as-contextual-notation.pdf)
- [Paths as Making Predictions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/paths-as-making-predictions.pdf)
- [Existential Paths as Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths-as-sets.pdf)

### Notation

- [Algebraic Notation for Asymmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/algebraic-notation-for-asymmetric-paths.pdf) ★★★★★
- [Cross Argument Asymmetric Path Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cross-argument-asymmetric-path-notation.pdf)
- [Path Function Product Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-function-product-notation.pdf) ★★★★★
- [Merging of Path Generators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/merging-of-path-generators.pdf) ★★★★★
- [Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths.pdf) ★★★★★
- [Domain Constraint Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/domain-constraint-notation.pdf) ★★★★★
- [Universal Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/universal-existential-paths.pdf) ★★★★★
- [Lambda Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/lambda-notation.pdf) ★★★★★
- [Function Currying Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/function-currying-notation.pdf) ★★★★★
- [Normal Paths as Function Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/normal-paths-as-function-sub-types.pdf)
- [Constrained Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-functions.pdf) ★★★★★
- [Sub-Type Aliasing](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/sub-type-aliasing.pdf) ★★★★★
- [The Id-Unit Function](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-id-unit-function.pdf)
- [Existence of Normal Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existence-of-normal-paths.pdf)
- [Dependent Asymmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/dependent-asymmetric-paths.pdf)
- [Adjoint Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adjoint-paths.pdf)
- [Adjoint Trivial Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adjoint-trivial-paths.pdf)

### Higher Order Operator Overloading

Higher Order Operator Overloading (HOOO) is a way to extend the semantics of normal expressions
into expressions for higher order reasoning.

- [Higher Order Operator Overloading](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading.pdf) ★★★★★
- [Higher Order Operator Overloading With Function Currying](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-with-function-currying.pdf)
- [Higher Order Operator Overloading for Mathematical Loops](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-for-mathematical-loops.pdf)
- [Higher Order Operator Overloading Lifting](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-lifting.pdf)
- [Higher Order Operator Overloading and Self Reference](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-self-reference.pdf)
- [Higher Order Operator Overloading and Partial Evaluation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-partial-evaluation.pdf)
- [Higher Order Operator Overloading and Natural Lambda Properties](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-natural-lambda-properties.pdf)
- [Higher Order Operator Overloading and Notation for Parameters](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-notation-for-parameters.pdf)
- [Higher Order Operator Overloading and Existential Path Equations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-existential-path-equations.pdf)
- [Higher Order Operator Overloading and Extensional Equality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-operator-overloading-and-extensional-equality.pdf)
- [Higher Order Operator Overloading by Explicit Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/higher-order-operator-overloading-by-explicit-notation.pdf) ★★★★★

### Discrete Path Semantics

- [Destructing Path Function Products](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/destructing-path-function-products.pdf)
- [Path Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-sets.pdf)
- [Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths.pdf)
- [Existential Path of Function Composition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-of-function-composition.pdf)
- [Existential Path of `if` expression](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-of-if-expression.pdf)
- [Universal Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/universal-existential-paths.pdf)
- [Constrained Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-functions.pdf)
- [Atomic Universal Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/atomic-universal-existential-paths.pdf)
- [Constrained Existential Path Implies Normal Path](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-existential-path-implies-normal-path.pdf)
- [Invertible Domain Constraints](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/invertible-domain-constraints.pdf)
- [Discrete Real Addition Isomorphisms](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/discrete-real-addition-isomorphisms.pdf)
- [Normal Re-paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/normal-re-paths.pdf)
- [Complexity of Symmetric Avatar Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/complexity-of-symmetric-avatar-paths.pdf)
- [Unary Embedding Termination](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/unary-embedding-termination.pdf)

### Boolean Path Semantics

- [Terminology for Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/terminology-for-binary-relations.pdf)
- [Symmetric Paths in Boolean Algebra](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetric-paths-in-boolean-algebra.pdf)
- [Adjoint Paths in Boolean Algebra](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adjoint-paths-in-boolean-algebra.pdf)
- [Mini Toolkit in Dyon for Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/mini-toolkit-in-dyon-for-boolean-path-semantics.pdf)
- [Existential Path in Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-in-boolean-path-semantics.pdf)
- [Logical Interpretations of Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/logical-interpretations-of-boolean-path-semantics.pdf)
- [Set Theory vs Boolean Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/set-theory-vs-boolean-functions.pdf)
- [Higher Order De Morgan's Laws for Unary Binary Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-de-morgans-laws-for-unary-binary-functions.pdf)
- [Union of Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/union-of-existential-paths.pdf)
- [Material Implications from Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/material-implications-from-existential-paths.pdf)
- [Countable Infinity from Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/countable-infinity-from-existential-paths.pdf)
- [And-Not-Or Set Matrix](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/and-not-or-set-matrix.pdf)
- [Communicated Concreteness](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/communicated-concreteness.pdf)

### Permutative Path Semantics

- [Permutative Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutative-functions.pdf)
- [Permutative Binary Numbers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutative-binary-numbers.pdf)
- [Generalized Swap Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/generalized-swap-grammar.pdf)
- [Sub-Permutation Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/sub-permutation-grammar.pdf)
- [Branch Permutation Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/branch-permutation-grammar.pdf)
- [Permutative Abacaba Vector](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutative-abacaba-vector.pdf)
- [Ordered Swaps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/ordered-swaps.pdf)
- [Permutation Group of Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutation-group-of-functions.pdf)
- [Encoding Equivalences as Swaps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/encoding-equivalences-as-swaps.pdf)
- [Equivalences Between Equivalences as Swaps of Swaps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/equivalences-between-equivalences-as-swaps-of-swaps.pdf)
- [Swaps of Swaps Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/swaps-of-swaps-grammar.pdf)
- [Uncovered Permutation Grammars](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/uncovered-permutation-grammars.pdf)
- [Permutations as Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutations-as-functions.pdf)
- [Permutation Inverse](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutation-inverse.pdf)
- [Permutation Intersections](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutation-intersections.pdf)
- [Visual Formula for Determinants](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/visual-formula-for-determinants.pdf)
- [Swap-Contract Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/swap-contract-graphs.pdf)
- [Permutative Symmetry Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutative-symmetry-paths.pdf)
- [Permutations as Cubical Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/permutations-as-cubical-paths.pdf)
- [Glubical Permutations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/glubical-permutations.pdf)

### Examples of Path Semantics

- [Existential Path of `add` with Domain Constraint `even`](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-of-add-with-domain-constraint-even.pdf)
- [Formalization of Tic-Tac-Toe in Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/formalization-of-tic-tac-toe-in-path-semantics.pdf)
- [Linear Natural Number Sequences](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/linear-natural-number-sequences.pdf)
- [Algebraic Properties of Addition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/algebraic-properties-of-addition.pdf)
- [Formalizing Climate Change Mitigation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/formalizing-climate-change-mitigation.pdf)
- [Existential Path of Left Recursive Addition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-of-left-recursive-addition.pdf)
- [Monotonic Properties of Lists](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/monotonic-properties-of-lists.pdf)
- [Parity of General Complex Numbers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/parity-of-general-complex-numbers.pdf)
- [Normal Path Diagram of AND](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/normal-path-diagram-of-and.pdf)

### Concepts in Path Semantics

- [Function Identity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/function-identity.pdf) ★★★★★
- [The Terminal Function & Propositions of Irrelevance](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-terminal-function-and-propositions-of-irrelevance.pdf)
- [Natural Equality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-equality.pdf)
- [Asymmetric Inverse](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/asymmetric-inverse.pdf)
- [Associativity as Asymmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/associativity-as-asymmetric-paths.pdf)
- [Composite & Prime Numbers in Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/composite-and-prime-numbers-in-path-semantics.pdf)
- [Inverse Zero Order Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/inverse-zero-order-existential-paths.pdf)
- [Structure Preserving Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/structure-preserving-functions.pdf)
- [Variables as Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/variables-as-path.pdf)
- [Conditions (Universal Existential Path is the Opposite Concept of Function Composition That Construct Ordinary Conditions)](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/universal-existential-path-is-the-opposite-concept-of-function-composition-that-constructs-ordinary-conditions.pdf)
- [Intervals on Projective Reals](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/intervals-on-projective-reals.pdf)
- [Entangled Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/entangled-functions.pdf) ★★★★★
- [Entangled Functions in Boolean Algebra](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/entangled-functions-in-boolean-algebra.pdf) ★★★★★
- [Law of Excluded Middle in Havox Diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/law-of-excluded-middle-in-havox-diagrams.pdf)
- [Formalizing the Meaning of Solving an Equation for One Variable](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/formalizing-the-meaning-of-solving-an-equation-for-one-variable.pdf)
- [Closed Sub-Types of Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/closed-sub-types-of-sets.pdf)
- [Uniform Properties of Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/uniform-properties-of-sets.pdf)
- [Constrained Uniform Properties of Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-uniform-properties-of-sets.pdf)
- [Liar's Paradox and Complete Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/liars-paradox-and-complete-functions.pdf)
- [Dependent L-System](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/dependent-lsystem.pdf)
- [Information Optimal Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/information-optimal-functions.pdf)
- [The Meaning of Consistency](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-meaning-of-consistency.pdf)
- [Natural Numbers by Coinductive Construction](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-numbers-by-coinductive-construction.pdf)
- [Spatial Sum Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/spatial-sum-functions.pdf)
- [Argument Relevant Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/argument-relevant-functions.pdf)
- [Argument Irrelevant Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/argument-irrelevant-functions.pdf)
- [Full Binary Trees](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/full-binary-trees.pdf)
- [Rooted Full Binary Trees](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/rooted-full-binary-trees.pdf)
- [Infinite Complete Binary Trees](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/infinite-complete-binary-trees.pdf)
- [Infibin](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/infibin.pdf)
- [Semantics of Points](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/semantics-of-points.pdf)
- [Exponential Duplication](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/exponential-duplication.pdf)
- [Questioning the Notion of a Set](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/questioning-the-notion-of-a-set.pdf)
- [Unitary Symmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/unitary-symmetric-paths.pdf)
- [Symmetric Paths of Matrices](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetric-paths-of-matrices.pdf)
- [Symmetric Path of Function Composition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetric-path-of-function-composition.pdf)
- [Predicate Interpretation of Adjoint Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/predicate-interpretation-of-adjoint-paths.pdf)
- [Imaginary Inverse](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/imaginary-inverse.pdf) ★★★★★

### Commutativity

- [Non-Trivial Commutative Symmetry](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/non-trivial-commutative-symmetry.pdf)
- [Involution from Commutative Symmetry](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/involution-from-commutative-symmetry.pdf)
- [Trivial Commutative Symmetry](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/trivial-commutative-symmetry.pdf)
- [Adjoint Commutative Symmetry](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adjoint-commutative-symmetry.pdf)
- [Commutative Symmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/commutative-symmetric-paths.pdf)

### Calculus

- [Derivative](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/derivative.pdf)
- [Integral of Polynomial Product](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/integral-of-polynomial-product.pdf)
- [Integral of Dual Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/integral-of-dual-functions.pdf)
- [Bounded Integral](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/bounded-integral.pdf)
- [Monotonic Integral Transform](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/monotonic-integral-transform.pdf)

### Probabilistic Path Semantics

- [Assigning Probabilities to Boolean Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/assigning-probabilities-to-boolean-functions.pdf)
- [Assigning Probabilities to Logical Quantifiers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/assigning-probabilities-to-logical-quantifiers.pdf)
- [Probabilistic Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-existential-paths.pdf)
- [Probabilistic Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-sub-types.pdf)
- [Confidence Factors of Probabilistic Constraints](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/confidence-factors-of-probabilistic-constraints.pdf)
- [The `bits` Function Family](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-bits-function-family.pdf)
- [Constrained Power-Set Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constraint-power-set-notation.pdf)
- [Probabilistic Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-paths.pdf)
- [Probabilistic Paths as Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-paths-as-sub-types.pdf)
- [Obscure Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/obscure-functions.pdf)
- [Partial Diversity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/partial-diversity.pdf)

### Examples of Probabilistic Path Semantics

- [Probabilistic Path of If-Expressions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-path-of-if-expressions.pdf)
- [Practice Problem 1 for Probabilistic Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/practice-problem-1-for-probabilistic-paths.pdf)
- [Probabilistic Liar's Paradox](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/probabilistic-liars-paradox.pdf)
- [Predicting Primality of Addition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/predicting-primality-of-addition.pdf)
- [Ambiguous Probability of Random String Optimization](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/ambiguous-probability-of-random-string-optimization.pdf)

### Avatar Extensions

[Summary of Avatar Extensions](https://advancedresearch.github.io/avatar-extensions/summary.html)

Foundation of Avatar Logic: [Avatar Logic to Set Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatar-logic-to-set-theory.pdf) ★★★★★

Avatar Extensions is a technique of abstract generalization by exploiting symmetries inside "smaller" theories.
For an experimental implementation of Avatar Logic, see [Avalog](https://github.com/advancedresearch/avalog).
For visualization of Avatar Graphs, see [Avatar-Graph](https://github.com/advancedresearch/avatar_graph).

- [Introduction to Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/introduction-to-avatar-graphs.pdf)
- [Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-graphs.pdf) ★★★★★
- [Graph Interpretation of Avatar Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/graph-interpretation-of-avatar-logic.pdf) ★★★★★
- [Unique Universal Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/unique-universal-binary-relations.pdf) ★★★★★
- [Avatar Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-binary-relations.pdf) ★★★★★
- [Avatar Time Lines](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-time-lines.pdf)
- [Symmetric Avatar Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetric-avatar-paths.pdf)
- [Symmetry Forcing](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetry-forcing.pdf)
- [Avatar Covers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-covers.pdf)
- [Imaginary Adjoint Operators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/imaginary-adjoint-operators.pdf)
- [Split Adjoint Operators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/split-adjoint-operators.pdf)
- [Dual Adjoint Operators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/dual-adjoint-operators.pdf)
- [Natural One-Avatar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-one-avatar.pdf)
- [Associativity Avatar Graph](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/associativity-avatar-graph.pdf)
- [Avatar Algebra Superposition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-algebra-superposition.pdf)
- [Avatar Algebra Symmetry](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-algebra-symmetry.pdf)
- [Witness Duality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/witness-duality.pdf)
- [More About Witness Duality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/more-about-witness-duality.pdf)
- [Interior vs Exterior](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/interior-vs-exterior.pdf)
- [Exterior Non-Uniform Subjectivity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/exterior-non-uniform-subjectivity.pdf)
- [Naive Isomorphism Propagation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/naive-isomorphism-propagation.pdf)
- [Avatar Schema Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-schema-theory.pdf)
- [Maximum Mathematical Languages](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/maximum-mathematical-languages.pdf)
- [Synchronizability and Cosynchronizability](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/synchronizability-and-cosynchronizability.pdf)
- [Restricted Dual Composition](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/restricted-dual-composition.pdf)
- [Dual Identity Lift](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/dual-identity-lift.pdf)
- [Avatar Univalence](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatar-univalence.pdf) ★★★★★
- [Univalent Involutions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/univalent-involutions.pdf)
- [The Subjective Sense of IO](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-subjective-sense-of-io.pdf)
- [Avatars in The Holy Trinity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatars-in-the-holy-trinity.pdf)
- [Avatar Tables](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatar-tables.pdf)
- [Symbiosis and Ourobiosis](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/symbiosis-and-ourobiosis.pdf) ★★★★★
- [Ourobra Tables](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/ourobra-tables.pdf)
- [Symbolic Distinction](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/symbolic-distinction.pdf) ★★★★★
- [Path Semantical Quality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/path-semantical-quality.pdf) ★★★★★
- [Zen Languages](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/zen-languages.pdf)
- [Avatar Zen Ladders](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatar-zen-ladders.pdf)
- [Qual and Qualitative](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/qual-and-qualitative.pdf)
- [Seshatism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/seshatism.pdf) ★★★★★
- [Path Semantical Quality Truth Table](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/path-semantical-quality-truth-table.pdf)
- [Last Order Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/last-order-logic.pdf)
- [Seshatic-Platonic Cycles](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/seshatic-platonic-cycles.pdf)
- [Consciousness in Wolfram Models](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/consciousness-in-wolfram-models.pdf) ★★★★★
- [Joker Calculus](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/joker-calculus.pdf) ★★★★★
- [Essay: What is Wrong with Platonism?](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/what-is-wrong-with-platonism.pdf) ★★★★★
- [Avatar Hypergraph Rewriting](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/avatar-hypergraph-rewriting.pdf) ★★★★★
- [The Symbolic Distinction Hypothesis of Consciousness](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-symbolic-distinction-hypothesis-of-consciousness.pdf) ★★★★★
- [The Joker Type](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-joker-type.pdf) ★★★★★
- [The Rise of Seshatism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-rise-of-seshatism.pdf) ★★★★★
- [Nuobocat](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/nuobocat.pdf)
- [Seshatic Inequality Overloading](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/seshatic-inequality-overloading.pdf)
- [Essay: Constructive Critique of Buddha](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/constructive-critique-of-buddha.pdf)
- [Logi](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/logi.pdf) ★★★★★
- [Seshatic and Platonic Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/seshatic-and-platonic-paths.pdf) ★★★★★
- [Seshatic Queenity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/seshatic-queenity.pdf)
- [Natural Univalent Calculus](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/natural-univalent-calculus.pdf) ★★★★★
- [Cube of Elementary Systems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/cube-of-elementary-systems.pdf)
- [Existential Propositions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/existential-propositions.pdf) ★★★★★
- [Cogito Cross Equality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/cogito-cross-equality.pdf) ★★★★★
- [Martin Heidegger vs Alan Watts](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/martin-heidegger-vs-alan-watts.pdf)

#### Conversations With GPT-3

- [Conversations With GPT-3](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/conversations-with-gpt-3.pdf)
- [Conversations With GPT-3 Part 2](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/conversations-with-gpt-3-part-2.pdf)

### Homotopy Physics

Homotopy Physics applies the theory of Avatar Extensions to Quantum Physics.

- [Homotopy Physics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/homotopy-physics.pdf)
- [Homotopy Physics and Path Integral Formulation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/homotopy-physics-and-path-integral-formulation.pdf)
- [Homotopy Physics and Derivation of Homotopy Maps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/homotopy-physics-and-derivation-of-homotopy-maps.pdf)
- [Homotopy Physics and Avatars](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/homotopy-physics-and-avatars.pdf)
- [Homotopy Physics and Simplex Approximations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/homotopy-physics-and-simplex-approximations.pdf)

Papers for the theoretical background to understand Homotopy Physics applied to experiments:

- [Linear Polarized Light](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/linear-polarized-light.pdf)

### Proof Techniques

- [Type Checking Variable With Multiple Constraints](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/type-checking-variable-with-multiple-constraints.pdf)
- [Reduction of Proofs With Multiple Constraints](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/reduction-of-proofs-with-multiple-constraints.pdf) ★★★★★
- [Strategies for Analyzing Data About Symmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/strategies-for-analysing-data-about-symmetric-paths.pdf)
- [Equation Inference](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/equation-inference.pdf)
- [Proving Equality in Two Different Ways](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proving-equality-in-two-different-ways.pdf)
- [Proving Non-Existence of Symmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proving-non-existence-of-symmetric-paths.pdf)
- [Proving Non-Existence of Asymmetric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proving-non-existence-of-asymmetric-paths.pdf)
- [Detecting Existence of Asymmetric Boolean Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/detecting-existence-of-asymmetric-boolean-paths.pdf)
- [Encoding Knowledge About Existential Paths as Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/encoding-knowledge-about-existential-paths-as-functions.pdf)
- [Finding Normal Paths Using Zero Order Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/finding-normal-paths-using-zero-order-existential-paths.pdf)
- [Quality of Search Data for Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quality-of-search-data-for-paths.pdf)
- [Solvable Equations = Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/solvable-equations-are-functions.pdf)
- [Machine Learning of Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/machine-learning-of-existential-paths.pdf)
- [Law of Sub-Type Extension](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/law-of-sub-type-extension.pdf)
- [Law of Sub-Type Reduction by Transitivity Variable Elimination](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/law-of-sub-type-reduction-by-transitivity-variable-elimination.pdf)
- [Law of Sub-Solution-Type Problem Simplification](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/law-of-sub-solution-type-problem-simplification.pdf)
- [Proof of Exclusiveness](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proof-of-exclusiveness.pdf)
- [Havox diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/havox-diagrams.pdf)
- [Surprising Strong Assumptions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/surprising-strong-assumptions.pdf)
- [Context Theorem Proving](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/context-theorem-proving.pdf)
- [Detecting Relevant Arguments in Boolean Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/detecting-relevant-arguments-in-boolean-functions.pdf)
- [Missing Assumption Search](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/missing-assumption-search.pdf)
- [Alpha Theorem Proving](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/alpha-theorem-proving.pdf)
- [Proof Systems as Grammars](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proof-systems-as-grammars.pdf)
- [Contracting Havox Diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/contracting-havox-diagrams.pdf)
- [Path Types in Tree Proofs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-types-in-tree-proofs.pdf)
- [Natural Numbers Constructed by Contracted Havox Diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-numbers-constructed-by-contracted-havox-diagrams.pdf)
- [Decidability in Constrained Universal Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/decidability-in-dependently-constrained-universal-existential-paths.pdf)
- [Mini Toolkit in Dyon for Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/mini-toolkit-in-dyon-for-boolean-path-semantics.pdf)
- [Existential Path in Boolean Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-path-in-boolean-path-semantics.pdf)
- [Constrained Existential Path Implies Normal Path](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-existential-path-implies-normal-path.pdf)
- [Proof of Equivalence](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proof-of-equivalence.pdf)
- [Skolem and Herbrand Normal Form](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/skolem-and-herbrand-normal-form.pdf)
- [Parameter Elimination in Higher Order Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/parameter-elimination-in-higher-order-existential-paths.pdf)
- [Path Operators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-operators.pdf)
- [Truth Values of Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/truth-values-of-sub-types.pdf)
- [Identity Proofs of Higher Homotopy - Explained for Programmers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/identity-proofs-of-higher-homotopy-explained-for-programmers.pdf)
- [Cartesian Products vs Arrows](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cartesian-products-vs-arrows.pdf)
- [Propositional Logic as Symbolic Free Form Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/propositional-logic-as-symbolic-free-form-grammar.pdf)
- [Efficient Type Checking](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/efficient-type-checking.pdf)
- [Equations as Algebraic Objects](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/equations-as-algebraic-objects.pdf)
- [Natural Implication](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-implication.pdf)
- [Natural Likely](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-likely.pdf)
- [Natural Sum Sub-Type Factorization](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-sum-sub-type-factorization.pdf)
- [Informal Theorem Proving](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/informal-theorem-proving.pdf)
- [Proof Optimization by Uniqueness](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/proof-optimization-by-uniqueness.pdf)
- [Logical Properties of Propositional Logic Systems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/logical-properties-of-propositional-logic-systems.pdf)
- [Type Inhabitation as Existence of Normal Identity Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/type-inhabitation-as-existence-of-normal-identity-paths.pdf)
- [Representation of Trivial Path in AST](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/representation-of-trivial-path-in-ast.pdf)
- [Cubical Binary Codes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cubical-binary-codes.pdf)
- [Time-Travel Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/time-travel-logic.pdf)
- [The Scientific Method and Language Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-scientific-method-and-language-semantics.pdf)
- [Decidability by Frequency Propositions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/decidability-by-frequency-propositions.pdf)
- [Undecidable Infinitesimals](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/undecidable-infinitesimals.pdf)
- [Wildcard Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/wildcard-sets.pdf)
- [Single Variable First-Order Proof Transform Into Propositional Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/single-variable-first-order-proof-transform-into-propositional-logic.pdf)
- [Constrained Normal Path Proofs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-normal-path-proofs.pdf)
- [Monotonic Non-Linear Solutions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/monotonic-non-linear-solutions.pdf)
- [Stair Pairs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/stair-pairs.pdf)
- [Cartesian Stair Pair Combinatorics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cartesian-stair-pair-combinatorics.pdf)
- [Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-graphs.pdf) ★★★★★
- [Unique Universal Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/unique-universal-binary-relations.pdf)
- [Abstract Idempotent Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/abstract-idempotent-functions.pdf)
- [Emph Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/emph-notation.pdf)
- [Semantic Complexity of Binary Relations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/semantic-complexity-of-binary-relations.pdf)
- [Role Lists](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/role-lists.pdf)
- [Incomplete Proof Search](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/incomplete-proof-search.pdf)
- [String Rewriting Rules for Cartesian Combinatorics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/string-rewriting-rules-for-cartesian-combinatorics.pdf)
- [Matrix Tangent Space](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/matrix-tangent-space.pdf)
- [Continuous Monotonicity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/continuous-monotonicity.pdf)
- [Hyperreal Progression Circle](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/hyperreal-progression-circle.pdf)
- [Invertible Adjoint Normal Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/invertible-adjoint-normal-paths.pdf)
- [Uniqueness in Single-Variable Proofs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/uniqueness-in-single-variable-proofs.pdf)
- [Discrete Lattice Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/discrete-lattice-functions.pdf)
- [Continuous Lattice Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/continuous-lattice-functions.pdf)
- [Propositional Argument Order](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/propositional-argument-order.pdf)
- [Normal Path Diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/normal-path-diagrams.pdf)
- [Cocategory Enumeration](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocategory-enumeration.pdf)

### Sized Type Theory

- [Sized Type Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/sized-type-theory.pdf)
- [Algebraic Sized Type Constructors](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/algebraic-sized-type-constructors.pdf)
- [Examples of Sized Type Equivalence](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/examples-of-sized-type-equivalence.pdf)
- [Counter-Factual Equality of Primes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/counter-factual-equality-of-primes.pdf)
- [Encoding Equivalences as Swaps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/encoding-equivalences-as-swaps.pdf)
- [Equivalences Between Equivalences as Swaps of Swaps](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/equivalences-between-equivalences-as-swaps-of-swaps.pdf)
- [Swaps of Swaps Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/swaps-of-swaps-grammar.pdf)
- [Modal Logic of Equivalences](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/modal-logic-of-equivalences.pdf)

### MX Grammar for Dependent Types

- [MX Grammar for Dependent Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/mx-grammar-for-dependent-types.pdf)
- [Variables in MX Dependent Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/variables-in-mx-dependent-types.pdf)
- [Evaluation of Variable in MX Dependent Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/evaluation-of-variable-in-mx-dependent-types.pdf)
- [Evaluation of Membership in MX Dependent Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/evaluation-of-membership-in-mx-dependent-types.pdf)

### Slot Lambda Calculus

- [Slot Lambda Calculus](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/slot-lambda-calculus.pdf)
- [Boolean Algebra Using If-Else](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/boolean-algebra-using-if-else.pdf)

### Dual Number Monotonic Density

- [Dual Number Monotonic Density](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/dual-number-monotonic-density.pdf)
- [Discrete Monotonic Limits](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/discrete-monotonic-limits.pdf)

### Cocyclic Graphs

- [Cocyclic Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-graphs.pdf)
- [Cocyclic Squares](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-squares.pdf)
- [Cocyclic Cubes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-cubes.pdf)
- [Cocyclic Pentagons](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-pentagons.pdf)
- [Cocyclic Hexagons](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-hexagons.pdf)
- [Cocyclic N-gon Necklaces](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cocyclic-n-gon-necklaces.pdf)

### Directional Set Algebra

- [Directional Set Algebra](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/directional-set-algebra.pdf)
- [Directional Set Products](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/directional-set-products.pdf)
- [Binary Square Matrix Combinatorics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/binary-square-matrix-combinatorics.pdf)

### Closed Natural Numbers

- [Closed Natural Numbers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/closed-natural-numbers.pdf) ★★★★★
- [Closed Natural Numbers and The Fundamental Identity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/closed-natural-numbers-and-the-fundamental-identity.pdf)
- [Closed Natural Numbers Cosmology](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/closed-natural-numbers-cosmology.pdf)
- [Hilbert's Over-Filled Hotel](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/hilberts-over-filled-hotel.pdf)
- [Natural Univalent Calculus](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/natural-univalent-calculus.pdf) ★★★★★

### Asymmetric Velocity Logic

- [Asymmetric Velocity Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/asymmetric-velocity-logic.pdf)
- [Billiard Balls in Asymmetric Velocity Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/billiard-balls-in-asymmetric-velocity-logic.pdf)
- [Simple Model of Asymmetric Velocity](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/simple-model-of-asymmetric-velocity.pdf)

### Answered Modal Logic

- [Avatar Graphs](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/avatar-graphs.pdf) ★★★★★
- [Symmetric Avatar Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symmetric-avatar-paths.pdf)
- [Cubical Binary Codes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cubical-binary-codes.pdf)
- [Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/answered-modal-logic.pdf) ★★★★★
- [Answered Modal Logic in Cubical Binary Codes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/answered-modal-logic-in-cubical-binary-codes.pdf)
- [Canonical Form of Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/canonical-form-of-answered-modal-logic.pdf)
- [Extracting Bits in Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/extracting-bits-in-answered-modal-logic.pdf)
- [Implication House](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/implication-house.pdf)
- [Propositional Logic Interpretation of Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/propositional-logic-interpretation-of-answered-modal-logic.pdf)
- [Visualizing Inversion vs NOT in Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/visualizing-inversion-vs-not-in-answered-modal-logic.pdf) ★★★★★
- [AND with Cubical Binary Codes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/and-with-cubical-binary-codes.pdf)
- [Truth Tables for Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/truth-tables-for-answered-modal-logic.pdf) ★★★★★
- [Answered Modal Logic Catuṣkoṭi](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/answered-modal-logic-catuskoti.pdf) ★★★★★
- [Recursive Booleans](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/recursive-booleans.pdf)
- [Chirality in Answered Modal Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/chirality-in-answered-modal-logic.pdf)
- [Many-Value Logics and Involutions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/many-value-logics-and-involutions.pdf)
- [Catuṣkoṭi Communication](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/catuskoti-communication.pdf)
- [Catuṣkoṭi Existential Path Equations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/catuskoti-existential-path-equations.pdf)
- [Catuṣkoṭi Existential Lift](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/catuskoti-existential-lift.pdf)
- [The Joker](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-joker.pdf) ★★★★★
- [The Subjective Sense of IO](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/the-subjective-sense-of-io.pdf)

### Theories

- [Modification Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/modification-theory.pdf)
- [Perfect Prime Predictors](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/perfect-prime-predictors.pdf)
- [Perfect Physical Predictors](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/perfect-physical-predictors.pdf)
- [Theorem Prover Combinators](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/theorem-prover-combinators.pdf)
- [Efficient Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/efficient-sets.pdf)
- [Universally Optimal Compression](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/universally-optimal-compression.pdf)
- [Local Optimal Safety Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/local-optimal-safety-theory.pdf)
- [Subset Sentences](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/subset-sentences.pdf)
- [Real Fractal Meta Probability Theory](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/real-fractal-meta-probability-theory.pdf)
- [Modal Logic of Observations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/modal-logic-of-observations.pdf)
- [Category Theory of Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/category-theory-of-logic.pdf)
- [Simple Structure Logic](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/simple-structure-logic.pdf)
- [One-Dimensional Real Wave](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/one-dimensional-real-wave.pdf)
- [Constant Speed Transform](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constant-speed-transform.pdf)
- [Single-Bit Languages](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/single-bit-languages.pdf)
- [Constructive Symmetry Breaking](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constructive-symmetry-breaking.pdf)
- [Segmented Time](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/segmented-time.pdf)
- [Listing-Möbius Shifts](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/listing-mobius-shifts.pdf)
- [Algexenotation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/algexenotation.pdf)

### Artificial Intelligence

[Moved to its own sequences page](./ai-sequences.md)

### Esoteric Path Semantics

- [Properties of Non-Constructive Objects in Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/properties-of-non-constructive-objects.pdf)
- [Intentional Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/intentional-paths.pdf)
- [Higher Kinded Embodiments](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-kinded-embodiments.pdf)
- [Isomorphic and Homotopy Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/isomorphic-and-homotopy-paths.pdf)
- [Transitive Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/transitive-existential-paths.pdf)
- [Differential Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/differential-existential-paths.pdf)
- [Homotopy Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/homotopy-sub-types.pdf)
- [Properties of Consciousness, Qualia and Self-Reflection by Intentional Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/properties-of-consciousness-qualia-and-self-reflection-by-intentional-paths.pdf)
- [Abstract Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/abstract-sub-types.pdf)
- [Abstract Assertions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/abstract-assertions.pdf)
- [Adjective Sub-Types](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adjective-sub-types.pdf)
- [Symbolent Calculus of Symbolic Type Inference](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/symbolent-calculus-of-symbolic-type-inference.pdf)
- [Serebin Path Tree](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/serebin-path-tree.pdf)
- [Equivalence Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/equivalence-paths.pdf)
- [General Existential Path of Language](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/general-existential-path-of-language.pdf)
- [Adversarial Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adversarial-paths.pdf)
- [Adversarial Discrete Topology](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adversarial-discrete-topology.pdf)
- [Anthropic Quantum Theorems](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/anthropic-quantum-theorems.pdf)
- [Adversarial Path of List](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adversarial-path-of-list.pdf)
- [Adversarial Path of Cartesian Product](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adversarial-path-of-cartesian-product.pdf)
- [Adversarial Path of Fuel Distance](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/adversarial-path-of-fuel-distance.pdf)
- [Cartesian Outer Product of Adversarial Path of List](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/cartesian-outer-product-of-adversarial-path-of-list.pdf)
- [Geometric Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/geometric-paths.pdf)
- [Natural Continuous Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-continuous-paths.pdf)
- [The Inquiry for Semantics of Choices](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-inquiry-for-semantics-of-choices.pdf)
- [Identity of Words in Natural Languages](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/identity-of-words-in-natural-languages.pdf)
- [Bootstrapping Technique for Semantics of Natural Languages](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/bootstrapping-technique-for-semantics-of-natural-languages.pdf)
- [The Two-Language Hypothesis](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/the-two-language-hypothesis.pdf)
- [Hamming N-Sphere](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/hamming-n-sphere.pdf)
- [Golden Measure](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/golden-measure.pdf)
- [3-ary Collatz Grammar](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip2/3-ary-collatz-grammar.pdf)

### Non-Deterministic Path Semantics

- [Non-Deterministic Existential Paths](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/non-deterministic-existential-paths.pdf)
- [Current Object Constrain Notation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/current-object-constraint-notation.pdf)
- [Existential Paths of Function Sets](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths-of-function-sets.pdf)
- [Universal Non-Deterministic Sampler](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/universal-non-deterministic-sampler.pdf)
- [Liar's Paradox and Complete Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/liars-paradox-and-complete-functions.pdf)
- [Natural Number Fuzzing](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-number-fuzzing.pdf)
- [Randomary Numbers](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/randomary-numbers.pdf)
- [Randomary Nth Contractibility](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/randomary-nth-contractibility.pdf)
- [Partial Observations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/partial-observations.pdf)
- [Quantum Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-non-determinism.pdf)
- [Non-Constructibility of Quantum Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/non-constructibility-of-quantum-non-determinism.pdf)
- [All Single Qubits are Constructible](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/all-single-qubits-are-constructible.pdf)
- [Order-Free Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/order-free-non-determinism.pdf)
- [Order-Free Quantum Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/order-free-quantum-non-determinism.pdf)
- [Quantum Knight Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-knight-functions.pdf)
- [Semi Quantum Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/semi-quantum-non-determinism.pdf)
- [Quantum Andor Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-andor-functions.pdf)
- [Quantum Propagation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-propagation.pdf)
- [Higher Order Non-Determinism](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-non-determinism.pdf)
- [Higher Order Non-Deterministic Diagrams](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/higher-order-non-deterministic-diagrams.pdf)
- [Instant Quantum Partial Observations](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/instant-quantum-partial-observations.pdf)
- [Quantum Information Flux](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-information-flux.pdf)
- [Quantum Information Flux of Rotating Complex Amplitudes](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-information-flux-of-rotating-complex-amplitudes.pdf)
- [Quantum Schrödinger Functions](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-schrodinger-functions.pdf)
- [Quantum Lift](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-lift.pdf)
- [Natural Frequency](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/natural-frequency.pdf)
