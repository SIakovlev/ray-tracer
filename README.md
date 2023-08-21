# Ray Tracer

## Introduction

Simple Ray Tracer implementation written in Rust.

The goal of the project is to learn/explore Rust, its ecosystem, and develop something new, complex and fun.

![Sphere, cube and two planes. One of them is a mirror](./imgs/basic_shapes.jpg)

## Getting Started

### Tests

Each component of the project is fully covered with unit tests. To run tests:

- `cargo test` to run tests

### Build

Use one of the following commands to build:

- `cargo build` - debug target
- `cargo build --release` - release target

### Run

- `cargo run --release` - run release target. Current implementation generates `.ppm` files. They can be opened with default tools on MacOS. I have not tested it on other platforms yet.

### References

- The structure and implementation loosely follows [The Ray Tracer Challenge](http://raytracerchallenge.com/) book by Jamis Buck. The book provides a very good introduction into how a ray tracer works leaving implementation details to the reader.
- Many reflection and refraction formulas are taken from [here](https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf)

### TODO

- More RT related stuff:
  - More geometries: cubes, triangles, cylinders. Generalisation.
  - Complex/Solid geometries support
  - OBJ files
- CLI
- Parallel implementation w/ Rayen
