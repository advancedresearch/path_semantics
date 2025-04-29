# Frequently Asked Questions

Here is list of questions and answers that you might have about Path Semantics.

### What should I learn to understand Path Semantics?

- Basic Category Theory (e.g. what is commutative squares)
- Basic Homotopy Type Theory (e.g. what is a homotopy)

For Avatar Extensions, it is useful to know something about Basic Group Theory.

### What is the overall problem?

Dependent types can encode extra information that is used to check behaviour of programs statically.
For example, Coq, which uses Calculus of Constructions, is used to verify software and proofs in mathematics.
However, dependent types is not a solution once-and-for-all and recent years there has been an explosion of research
into Cubical Type Theory, to make theorem proving easier and more constructive.

When talking about extending dependent types, it is important to distinguish between objects that are formulated
within some language and objects that are first class citizens.
It is easier to provide good support for objects as first class citizens.
This is a matter of usability, not about provability.

Path Semantics try to address one of these problems of usability,
which might be thought of as introducing a new first class citizen called a "normal path".
There is a lot of technical details involved in this work and most of it does not matter for other fields of mathematics.
However, it turns out that the logical foundation that formalizes what should be provable about normal paths,
might have surprising relations to remotely associated fields such as Continental Philosophy.

### Why normal paths?

For highly technical reasons, it is not sufficient to use syntax sugar for normal paths.
Normal paths need to be considered mathematical objects on their own, kind of like complex numbers vs real numbers.
The idea is to improve the process of theorem proving itself, but this does not change which theorems can be proved.

### What is a "path"?

You can read the formal definition of normal paths [here](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/formal-definition-of-normal-paths.pdf).

Assume you have two continuous functions `f` and `g`.
Formally, a homotopy path `H` is a continuous function of type `X x [0, 1] -> Y` where `H(x, 0) = f(x)` and `H(x, 1) = g(x)`.
You can read more about this [here](https://en.wikipedia.org/wiki/Homotopy).

A homotopy path is used in a field of mathematics called "topology",
but can also be used as a language building block to give people intuition about proofs, as developed in [Homotopy Type Theory](https://homotopytypetheory.org/).

In Homotopy Theory, a line can be contracted to a single point.
Formally, this means the following relation:

```
H(x, 0) = H(x, 1)
```

All values can be thought of as "points" in a space of their type.
When a path connects two values, one can think about it as a proof computing one value from the other.
Computing the same value for different inputs is equivalent to "contracting".

Just like you can take any two points and create a line between them,
you can create symbols and define logical relations between them.

A homotopy path encodes the relation using the unit interval `[0, 1]`.
In a sense, it formalizes the "meta-language" of proofs.

However, since all functions when used are existing in a real world,
the meta-language of proofs can be eliminated by refering to the "associated meaning" of symbols.

For example, when you perform a computation, the physical computation itself provides the "proof", or the "path".
When two functions are related to each other through a logical definition, one can use the relation
to refer to the abstract "proof" or "path".

Path Semantics takes the intuition of homotopy paths and eliminates the meta-language.
This means that homotopy paths are projected into ordinary types of functions.
Instead of `X x [0, 1] -> Y`, you get `X -> Y`, which is an ordinary function type.

For example, when one expresses `and[not] <=> or` in Path Semantics,
one refers to a way to prove that this is true.
Since `not` is bijective, one can say that `and` is "equivalent" to `or`, by the symmetric path `not`.

However, since homotopy paths can also contract lines into points,
it means that e.g. `concat[len] <=> add` describes a homotopy equivalence between `concat` and `add`.
The difference is that `add`, interpreted this way, is a contracted version of `concat`.

When working in Homotopy Type Theory, one is describing Path Semantics at the meta-level.
It is like looking at Path Semantics from the "outside".
In Path Semantics, one is "inside" the theory.

This is because mathematics is used "inside" the world, which makes it possible.
So, a "path" in Path Semantics is how one refers to the meaning, or semantics, of a logical relation between functions.

Hence, the name "Path Semantics".

### Is Path Semantics meant to be a new foundation of all mathematics?

No. It is meant to be a practical tool for reasoning about programs (e.g. deriving algorithms using math), but open-ended for reasoning with mathematics.

While it is true that the core axiom ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf)) might model logically how symbols are used for theorem proving in mathematics in general,
it provides no complete semantics of mathematics.

For every theory, the semantics of it must be formalized and defined before you can talk about what it is.

Think about this as a language:

If I say "I was home yesterday" you might ask "what did you do while at home?".
However, it makes no sense to ask what I did at home if I was not home.

Questions are meaningful only if they are asked in a context where answering it makes sense.

In a similar way, mathematics is a body of knowledge that is iterated over and improved over time.
There is no complete definition of mathematics, but there are some axioms that define rules of
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

### What is the difference between formal theorem proving and informal theorem proving?

Informal theorem proving permits usage of all formal languages for theorem proving.
No formal language can use all formal languages, therefore informal theorem proving is strictly more powerful than formal theorem proving ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/informal-theorem-proving.pdf)).

It does not mean that you are allowed to say anything you want in informal theorem proving.

What you are allowed to say in Path Semantics is determined by the core axiom ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics.pdf)).

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
because of side effects and non-determinism.
With other words, Set Theory is insufficient to deal with many problems that programmers want to solve.

Category Theory is viewing mathematics from a bird-view perspective, which is useful sometimes,
but understanding functions in more detail can be useful in ways where Category Theory does not fit solving the problem.

### How do you test that the ideas work?

By testing them in practice. After all, path semantics builds on the semantics of functions, which is available in all mainstream programming languages.

### How can you know that the ideas are correct?

Path semantics explains how to check that some ideas are correct, by writing programs.
This method assumes that you can implement those programs.

For example:

    a : [g] b
    
To check this, one must write the following check:

    b : [∃g] true
    
The definition of `∃g` follows from the definition of `g`, using the definition of an existential path ([link](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/existential-paths.pdf)).

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
If you do not trust functions, then you have problem.

Nobody have a full formal definition of functions as they are used in programming.
There are good reasons why: For example, arbitrary sub-types makes type checking undecidable.

Yet, functions are necessary because *they are used in programming*.

There are ways one can partially define formally the semantics of functions.
However, you can not expect this to be fully formalized, because it is impossible.
Therefore, Path Semantics is not fully formalizable.

### Is AdvancedResearch an organization for pseudo-mathematicians?

Pseudo-mathematics, or mathematical crankery, is a form of mathematics-like activity that aims at advancing a set of questionable beliefs that do not adhere to the framework of rigor or formal mathematical practice. Link to [Wikipedia article](https://en.wikipedia.org/wiki/Pseudomathematics)

Some people who criticize AdvancedResearch for this, usually in less nice words, often have the following lists of "merits":

- None or little Github activity
- Never gained an even basic understanding of automated theorem provers or propositional logic
- Lack basic knowledge about computational complexity theory
- Have never used software produced by AdvancedResearch
- Have never opened issued, visited forums or engaged meaningfully with AdvancedResearch

AdvancedResearch does research, therefore most of the documentation is for internal use.

Any claims in the past about pseudomathematics in the Path Semantics project,
have been checked and there is no evidence so far beyond normal research activity.
Criticism against the core science of Path Semantics in particular, has no merit.

According to [John Baez' Crackpot Index](https://math.ucr.edu/home/baez/crackpot.html),
the Path Semantics project scores -5 to 5,
depending on whether you consider offering prize money to find errors in the core axiom as valid,
which was found later and hence arguably showed it was meant to motivate constructive criticism.

This does not mean that AdvancedResearch does not commit mistakes, which is normal.

It is more likely, statistically, that some people on the Internet are just seeking to waste other people's time,
than for some open organization that produces software to deliberately design it based on pseudo-mathematics.
The software produced by AdvancedResearch is regularly reviewed for errors and any errors found are reported.
