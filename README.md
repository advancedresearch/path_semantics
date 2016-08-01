# path_semantics
A research project in path semantics, a re-interpretation of functions for expressing mathematics

### What is path semantics?

Functional programming has been an active research area for [dependently types](https://en.wikipedia.org/wiki/Dependent_type).
In this notation, a new semantics that re-interprets functions takes a step beyond dependently types.

See the [wiki](https://github.com/bvssvni/path_semantics/wiki) for more information.

### Is this a new programming language?

A new *kind* of programming language.

The project is about defining a new category of programming languages that satisfy a different method of expression compared to traditional languages.
The ideas are original, but inspired by recent advancement in type theory.

A short term goal is refining the semantics to satisfy the axioms.
Because it is based on a new method for type membership, it is easy to introduce inconsistency by applying existing techniques without testing.

A long term goal is to develop an interpreter and type system.
This is a very challenging problem, partly because the technique used can require execution to resolve function lookup.

### Previous work

Some ideas are taking from unpublished work. I have been asked to publish it but have not gotten time to do it yet.

Earlier, I explored ways to encode information into a generalized version of Adinkra diagrams to model states of discrete systems. The idea is since Adinkra diagrams are constructed by labeling the edges after specific rules, one could extract rules from edges of similar diagrams representing systems.

These diagrams have a reflective property that allows related concepts to be expressed with variations in a systematic way, intuitively described as "context modelling". This background knowledge, together with dependently type experiments, served as rationale for developing the notation.

While suitable to model context in various applications,
a problem is super exponential growth in memory usage.
Path semantics constructs a space that appear similar to the structure of such diagrams,
but compresses the information in a human readable form.

The diagrams have some important properties:

- Can be described fully using only an array of integers `[a, b, c, ...]`
- Uses direct group product as building block `[a, b] x [c, b] = [a, b, c, d]`
- The class with lowest complexity, `[2, 2, ...]`, are edges of hypercubes, as in Adinkra diagrams
- All diagrams are subset of itself in a single dimension `[N]`, which has the highest complexity

For algorithms to compute with these diagrams, see `Context` in the [discrete](https://github.com/bvssvni/discrete) library.
