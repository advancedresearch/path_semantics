# path_semantics
A research project in path semantics, a re-interpretation of functions for expressing mathematics

[Tutorial](https://github.com/advancedresearch/path_semantics/wiki/Tutorial-1:-Types)  
[A brief history of path semantics illustrated with stick figures](https://github.com/bvssvni/path_semantics/blob/master/papers-wip/history-of-path-semantics-illustrated.pdf)

### What is path semantics?

Here is a cheat sheet to show how it looks like: [Path Semantics Cheat Sheet](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/path-semantics-cheat-sheet.pdf)

Functional programming has been an active research area for [dependently types](https://en.wikipedia.org/wiki/Dependent_type).
In this notation, a new semantics that re-interprets functions takes a step beyond dependently types.

Very briefly, path semantics is about things like:

- How functions are constructed and connected
- How to express relationships between functions in a more strict way than equations
- What can be predicted about output of functions from something about the input
- What it means to refer to a function (function identity)
- What you can do with functions, given some class of knowledge about them is available
- What kind of structures are related to some class of functions

See the [wiki](https://github.com/advancedresearch/path_semantics/wiki) for more information.

### Is this a new programming language?

A new *kind* of programming language.

The project is about defining a new category of programming languages that satisfy a different method of expression compared to traditional languages.
The ideas are original, but inspired by recent advancement in type theory.

Goals:

- Create efficient algorithms that find paths
- Find deductive rules
- Find rules for well formed expressions
- Find applicable areas (machine learning etc.)
- Find generalizations (probability theory etc.)

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
