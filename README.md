# Rust Benchmarking Exercise:

## Overview

This is an exercise in learning more about Rust's benchmarking utilities.
I noticed the `bench` command after running `cargo help`, and thought that
the ability to easily benchmark code sounded interesting to me.

In a previous project, I had incidentally attempted to benchmark some of
my code manually, so this sounded like an interesting exercise. This
repository intends to document some introductory exercises in benchmarking
in Rust, visualizing that information, and other things that I learned along
the way.

## About Benchmarking

The command syntax functions as shown below. If this notation is unfamiliar
to you, it means that the options (if given) to the `bench` command are given
before the benchmark name (if given), and arguments to the program (if given)
is everything after a `--`.

```
USAGE:
    cargo bench [OPTIONS] [BENCHNAME] [-- <args>...]
```

The benchmarking command accepts a `BENCHNAME` string. If this is given, then
only benches containing this string in their name will be run. This is useful
to know about, it will allow us to run benchmarks against specific components.

## Project Overview

To practice benchmarking, we will need to come up with something to benchmark.
Ideally, we would like to compare the performance of alternative solutions to
a computional problem and, let's be honest, we are really here to make fancy
graphs, right?

### Matrix Multiplication

Matrix multiplication is a computationally intensive process, involving a
large number of operations to produce a result. There are some well known
optimizations to this process that we can implement, and compare against
the naive matrix multiplication algorithm.

We will write a small matrix multiplication library, that accepts two matrices
filled with numbers, and produces the product in the form of a new matrix.
Two matrix multiplication logic modules will be implemented, one using a
naive implementation, and another using an optimized matrix multiplication
algorithm.

Once this is built, we will benchmark the performance of the two alternatives.

## Lessons, Discoveries

### Problems Encountered

Learning more about how to compose `Option` types and `Result` types required
doing some further research. At this point in my Rust learning process, I was
finding myself still fighting against the borrow checker, and decided this
meant I should familiarize myself with common idiomatic error handling
patterns in Rust.

### Verify Project Correctness

`cargo verify-project` will check the correctness of the crate manifest.
If the project passed the check, you will see a result printed to output
like so:

```sh
✨  cargo verify-project
{"success":"true"}
```

