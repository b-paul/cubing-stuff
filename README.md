# cubing-stuff

This repository holds many of my cubing related experiments.
My [cube-lib](https://github.com/b-paul/cube-lib) crate is used by many of these programs.

- `drxs` is a DR minus x slice solver. It has an cli like `nissy`'s but solves only drxs.
- `generator-solver` searches for all states found within a move generator. This was used to create a quick list of all floppy reduction states when initial floppy reduction research was being performed.
- `pin-orders` is a reimplementation of the original 7-simul pin-set and pin-order search tool. It has support for determining which moves in a pin-order are intuitive (using fun `Z12` linear algebra algorithms), and is fast enough to analyse every pin-order (all permutations) in an acceptable amount of time.
