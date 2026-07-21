# rust-ray-tracer
A simple ray tracer with BVH acceleration implemented in Rust

## What
This project is a basic ray tracer with support for BVH acceleration, PBR materials, and basic lighting. It's designed to demonstrate the concepts of ray tracing and BVH acceleration in a simple, easy-to-understand way.

## Why
I built this project to learn about ray tracing and BVH acceleration, and to practice implementing complex algorithms in Rust. This project is a starting point for anyone looking to learn about these topics.

## Install
To run this project, you'll need Rust installed on your system. You can download the Rust installer from the official [Rust website](https://www.rust-lang.org/tools/install). Once you have Rust installed, you can build and run the project with the following commands:

```bash
cargo build
cargo run
```

## Usage
To use this project, simply run the `cargo run` command in the project directory. This will build and run the project, generating an image file in the `target` directory. You can customize the project's behavior by modifying the `src/main.rs` file.

## Build from Source
To build this project from source, simply clone the repository and run the `cargo build` command. This will build the project and generate a `target` directory containing the compiled binary and generated image file.

## Project Structure
The project is organized into the following directories:

* `src`: contains the source code for the project
* `tests`: contains unit tests for the project
* `docs`: contains documentation for the project (currently empty)

## License
This project is licensed under the [MIT License](https://opensource.org/licenses/MIT).

## Features
This project supports the following features:

* BVH acceleration
* PBR materials
* Basic lighting

## Dependencies
This project depends on the following libraries:

* `rayon` for parallelism
* `nalgebra` for linear algebra
* `image` for image processing

## Benchmarks
You can run the benchmarks with the following command:

```bash
cargo bench
```

This will run the benchmarks and generate a report in the `target` directory.