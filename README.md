# Rubik Master

[![Crates.io](https://img.shields.io/crates/v/rubikmaster.svg)](https://crates.io/crates/rubikmaster)
[![documentation](https://docs.rs/rubikmaster/badge.svg)](https://docs.rs/rubikmaster)
![CI](https://github.com/akiradeveloper/rubikmaster/workflows/CI/badge.svg)
[![Tokei](https://tokei.rs/b1/github/akiradeveloper/rubikmaster)](https://github.com/akiradeveloper/rubikmaster)

https://user-images.githubusercontent.com/785824/136357797-47672e97-09f2-4558-b4e2-7cfda34db298.mov

Do you like to solve Rubik's cube? I do.

As a cuber and programmer, I want to build a
toolset to build applications like

- Solver
- Virtual Cube (As WebGL component)
- Cube net printer
- Tool to find more ergonomic OLL/PLL
- (Semi-)Automatic scrambler

In this library, the state of the cube is expressed as
54x54 permutation matrix which consumes only 54 bytes in memory and
the multiplication costs only O(54) since the matrix is sparse.

This library will include the following modules:

- [x] Core: Matrix and Operations.
- [x] Parser: The rotation notations like (RUR')U'(R'FR)F' should be interpreted.
- [x] Virtual Cube component (WebGL and wasm): Will support features like animated rotation, guide arrow, camera move. **HELP WANTED**

I am open to any suggestions.

## Author

Akira Hayakawa (@akiradeveloper)