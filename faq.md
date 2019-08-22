# Frequently Asked Questions

Here is list of questions and answers that you might have about Path Semantics.

### Is Path Semantics meant to be a new foundation of all mathematics?

No. It is meant to be a practical tool for reasoning about programs, but open-ended for reasoning with mathematics.

While it is true that the core axiom ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf)) might describe how symbols are used for theorem proving in mathematics in general,
it provides no complete semantics of mathematics.

For every theory, the semantics of it must be formalized and defined before you can talk about what it is.

Think about this as a language:

If I say "I was home yesterday" you might ask "what did you do while at home?".
However, it makes no sense to ask what I did at home if I was not home.

Questions are meaningful only if they are asked in a context where answering it makes sense.

In a similar way, mathematics is a body of knowledge that is iterated over and improved over time.
There is no complete definition of mathematics, but there are some axioms that defines rules of
how to think about mathematics itself, abstractly.

For example, the univalence axiom of Homotopy Type Theory ([link](https://homotopytypetheory.org/)) can be written as a rule in
a theorem prover assistant. In that system, the proofs can be checked by a computer.
However, outside the system, the univalence axiom must be interpreted in some way to make sense.

So, when Path Semantics defines a core axiom, you must first find an environment to interpret what it means.
The environment will always contain a lot of complexity that is not built into the axiom.
This constraint prevents Path Semantics from being a foundation of all mathematics,
but for some environments, with an interpretation that does not violate the axiom,
it is indistinguishable from e.g. a whole theory of a field.

Path Semantics is used to reason about some parts of mathematics, not all of it.
It is much better to use existing theories and frameworks for various domains, than reconstructing it in Path Semantics.

### Why do you not prove that the core axiom works, why only a reward for disproving it?

If you can prove that the axiom works, then Path Semantics is not consistent, which is the same as disproving it.

This is because Path Semantics contains arithmetic.

You can read more about this [here (Gödel's incompleteness theorems)](https://en.wikipedia.org/wiki/G%C3%B6del%27s_incompleteness_theorems).

Proving soundness of a formal system is often done in formal languages for each term having a well-defined type.

Path Semantics is not a Type Theory. It is more fundamental.
Type Theory is added to Path Semantics through "bootstrapping".

You might be able to disprove the core axiom, by e.g. show that Type Theory as bootstrapped into gives a
different semantics than the one we use in formal systems.

### What is the difference between formal theorem proving and informal theorem proving?

Informal theorem proving permits usage of all formal languages for theorem proving.
No formal language can use all formal languages, therefore informal theorem proving is strictly more powerful than formal theorem proving ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/informal-theorem-proving.pdf)).

It does not mean that you are allowed to say anything you want in informal theorem proving.

What you are allowed to say in Path Semantics is determined by the core axiom ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf)).

### Why do you quote yourself?

It is a joke that refers to the stereotypical view of an amateur mathematician which makes other mathematicians call them a "crank".

If you call me a "crank" because I am quoting myself, your behavior is a proof of that joke.

Do you get the joke now? I am pretty sure you can work it out if you take the time. :)

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

In other languages, such as [Dependent L-System](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/dependent-lsystem.pdf), the existential path defined is an analogue of the existential path of functions.

### Why do you build upon functions as the basic idea and not e.g. Set Theory or Category Theory?

Functions are easier to understand for programmers.
Designing a language to talk about functions is also more useful for programmers.

The semantics of functions in mainstream programming is different from e.g. Set Theory,
because of side effects and indeterminism.
With other words, Set Theory is insufficient to deal with problems that programmers want to solve.

Category Theory is viewing mathematics from a bird-view perspective, which is useful sometimes,
but understanding functions in more detail can be useful in ways Category Theory does not fit.

### How do you test that the ideas work?

By testing them in practice. After all, path semantics builds on the semantics of functions, which is available in all mainstream programming languages.

### How can you know that the ideas are correct?

Path semantics explains how to check that some ideas are correct, by writing programs.
This method assumes that you can implement those programs.

For example:

    a : [g] b
    
To check this, one must write the following check:

    b : [∃g] true
    
The definition of `∃g` follows from the definition of `g`, using the definition of an existential path.

If you have a function which identity is used to talk about other functions,
then it is sufficient that the function is implemented.
This works because path semantics reduces `f[g] <=> h`.

### This is just another way of doing mathematics that is already done in practice! What is new?

The purpose of Path Semantics is to develop another way of doing mathematics that is already done in practice,
using a notation that feels more familiar and understandable for programmers.

Why? Because programmers like myself try to solve problems, and do not have time to follow mathematical journals.

There is no need to make new contributions to the existing mountain of knowledge in order to demonstrate that
a new way of expressing mathematical ideas using functions is more practical and easier to understand.

Techniques for helping to understand what mathematics is about, makes it easier for people to
utilize the existing mountain of knowledge. To do this, one must focus on some central ideas of importance.

### I do not trust Path Semantics without having a full formal definition. Why not make one?

Path Semantics build the core ideas on the semantics of functions, in order to avoid the need for a full formal definition.
If do not trust functions, then you have problem.

Nobody have a full formal definition of functions as they are used in programming.
There are good reasons why: For example, arbitrary sub-types makes type checking undecidable.

Yet, functions are necessary because *they are used in programming*.

There are ways one can partially define formally the semantics of functions.
However, you can not expect this to be fully formalized, because it is impossible.
Therefore, Path Semantics is not fully formalizable.

### Why do not use formal definitions?

I do. Everywhere. They are formally defined using functions.
