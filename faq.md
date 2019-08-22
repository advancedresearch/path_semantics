# Frequently Asked Questions

Here is list of questions and answers that you might have about Path Semantics.

### Is Path Semantics meant to be a new foundation of all mathematics?

No. While it is true that the axiom might describe how symbols are used for theorem proving in mathematics in general,
it provides no complete semantics of mathematics.

For every theory, the semantics of it must be formalized and defined before you can talk about what it is.

Think about this as a language:

If I say "I was home yesterday" you might ask "what did you do while at home?".
However, it makes no sense to ask what I did at home if I was not home.

Questions are meaningful only if they are asked in a context where answering it makes sense.

In a similar way, mathematics is a body of knowledge that is iterated over and improved over time.
There is no complete definition of mathematics, but there are some axioms that defines rules of
how to think about mathematics itself, abstractly.

For example, the univalence axiom of Homotopy Type Theory can be written as a rule in
a theorem prover assistant. In that system, the proofs can be checked by a computer.
However, outside the system, the univalence axiom must be interpreted in some way to make sense.

So, when Path Semantics defines an axiom, you must first find an environment to interpret what it means.
The environment will always contain a lot of complexity that is not built into the axiom.
This constraint prevents Path Semantics from being a foundation of all mathematics,
but for some environments, with an interpretation that does not violate the axiom,
it is indistinguishable from e.g. a whole theory of a field.

Path Semantics is used to reason about some parts of mathematics, not all of it.
It is much better to use existing theories and frameworks for various domains, than reconstructing it in Path Semantics.

### Why do you not prove that the axiom works, why only a reward for disproving it?

If you can prove that the axiom works, then Path Semantics is not consistent, which is the same as disproving it.

This is because Path Semantics contains arithmetic.

You can read more about this [here (Gödel's incompleteness theorems)](https://en.wikipedia.org/wiki/G%C3%B6del%27s_incompleteness_theorems).

Proving soundness of a formal system is often done in formal languages for each term having a well-defined type.

Path Semantics is not a Type Theory. It is more fundamental.
Type Theory is added to Path Semantics through "bootstrapping".

You might be able to disprove the axiom, by e.g. show that Type Theory as bootstrapped into gives a
different semantics than the one we use in formal systems.

### What is the difference between formal theorem proving and informal theorem proving?

Informal theorem proving permits usage of all formal languages for theorem proving.
No formal language can use all formal languages, therefore informal theorem proving is strictly more powerful than formal theorem proving ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/informal-theorem-proving.pdf)).

It does not mean that you are allowed to say anything you want in informal theorem proving.

What you are allowed to say in Path Semantics is determined by the core axiom ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf)).

### Why do you quote yourself?

Some people look for any excuse to call me a "crank", so I give them that excuse early on, so they don't have to read a lot to find one.

### Why do you not refer to papers e.g. listed in MathSciNet?

I have never used it. I do not follow mathematical journals.

My workflow is different from those of most mathematicians. See the section "Workflow" [here](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/about-publishing-path-semantics.pdf).

### Why do you not use terminology that other mathematicians are familiar with?

The central topic of path semantics is about functions.
To do this well, the terminology needs to be precise.

Path semantics is more strict than standard terminology, e.g.:

- Trivial path (a function describing the domain of a function, [link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/constrained-functions.pdf))
- Existential path (a function describing the codomain of a function, [link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths.pdf))

The way domain and codomain is used in standard terminology is as a set.
It is not clear that one is talking about functions of arbitrary sub-types.
The expression "domain constraint" might be used instead of "trivial path".
