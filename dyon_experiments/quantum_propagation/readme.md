# Quantum Propagation Explorer

[Quantum Propagation](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/quantum-propagation.pdf) is an algorithm that replaces
the Born rule for measurements with an infinite non-deterministic series (random paths) that converges to same probabilities.
The cover of random paths is a research topic in [Non-Deterministic Path Semantics](https://github.com/advancedresearch/path_semantics/blob/master/sequences.md#non-deterministic-path-semantics).

[Source](https://github.com/advancedresearch/path_semantics/blob/master/dyon_experiments/quantum_propagation/main.dyon)

For instructions, see comments in the source.

## Gallery

Here I post screenshots with formulas of interesting quantum functions.

The quantum function is `f` and the measurement is `g`.

- `g := and` is a shorthand for `g := \(a, b) = a && b`.
- `g := eq` is a shorthand for `g := \(a, b) = a == b`.

Each sample is a random path.

### Andor
![001.png](gallery/001.png)
- 20 000 samples
- `f := [(0, 1), (1, 0), (-1, 0), (0, 1)]`
- `g := and`

### Symmetric Andor
![002.png](gallery/002.png)
- 20 000 samples
- `f := [(0, 1), (2, 0), (-2, 0), (0, 1)]`
- `g := and`

### Secret Library (Random)
![003.png](gallery/003.png)
- 20 000 samples
- `f := [(0.53975284, 0.8347775), (-0.6443964, 0.28524667), (0.778826, 0.6626748), (0.84524584, 0.6032202)]`
- `g := eq`

### Gravitational Lock (Random)
![004.png](gallery/004.png)
- 20 000 samples
- `f := [(-0.69797146, -0.024551803), (-0.2906545, 0.67269856), (-0.99042594, -0.5189728), (-0.6754978, -0.5213077)]`
- `g := and`

### Eruption (Random)
![005.png](gallery/005.png)
- 20 000 samples
- `f := [(-0.5742652, -0.66125774), (0.5340726, -0.1768929), (-0.94883776, -0.21300188), (0.98182935, -0.90915376)]`
- `g := eq`

### Dusty Lens (Random)
![006.png](gallery/006.png)
- 20 000 samples
- `f := [(-0.18238503, -0.70073056), (0.7934853, -0.77827376), (0.46978903, 0.9165352), (-0.07867549, -0.4303975)]`
- `g := and`

### Geysir (Random)
![007.png](gallery/006.png)
- 20 000 samples
- `f := [(0.8935131, 0.9591373), (-0.05023841, 0.4859118), (-0.89832544, -0.49549758), (-0.5120574, -0.7996857)]`
- `g := eq`
