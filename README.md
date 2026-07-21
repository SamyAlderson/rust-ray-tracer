# Ray Tracer
A simple ray tracer with BVH acceleration implemented in Rust.

## What it does

This project implements a basic ray tracer with BVH (Bounded Volume Hierarchy) acceleration. It supports PBR materials and basic lighting. It's intended as a learning project to explore the basics of ray tracing and BVH acceleration.

## Installation

### Cargo.toml

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
rayon = "1.5.3"
```

Then run `cargo build` to build the project.

## Usage

Run the ray tracer with `cargo run`. You'll need to provide a scene file, which is a text file containing the scene description. You can use the `scenes` directory as a starting point.

## Building from source

Run `cargo build` to build the project. This will generate a `target` directory containing the compiled binary.

## Running tests

Run `cargo test` to run the test suite.

## Project structure

* `src/main.rs`: Main entry point of the program.
* `src/accel.rs`: BVH acceleration implementation.
* `src/materials.rs`: PBR material implementation.
* `src/scene.rs`: Scene representation and loading.
* `src/ray.rs`: Ray implementation.
* `src/light.rs`: Basic lighting implementation.
* `src/utils.rs`: Utility functions.
* `tests/test_scene.rs`: Test scene implementation.
* `tests/test_runner.rs`: Test runner implementation.

## License

Copyright (c) 2026 SamyAlderson

Licensed under the MIT License.