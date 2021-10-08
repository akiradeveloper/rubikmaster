# Rubik Master

[![Crates.io](https://img.shields.io/crates/v/rubikmaster.svg)](https://crates.io/crates/rubikmaster)
[![documentation](https://docs.rs/rubikmaster/badge.svg)](https://docs.rs/rubikmaster)
![CI](https://github.com/akiradeveloper/rubikmaster/workflows/CI/badge.svg)
[![Tokei](https://tokei.rs/b1/github/akiradeveloper/rubikmaster)](https://github.com/akiradeveloper/rubikmaster)

https://user-images.githubusercontent.com/785824/136539377-31b9fa00-2ad4-4048-9f3c-159b4d4b8114.mov

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

## Features

This library includes the following modules

- Core: The matrix representation of cube state and rotation.
- Parser: Parser for rotation notes like RUR'U'.
- Cube Component: Yew component to visualize a cube. Animation supported.

I am open to any suggestions.

## Author

Akira Hayakawa (@akiradeveloper)
