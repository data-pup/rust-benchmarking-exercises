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

## Implementation

Before we go any further, let's cover what simple matrix multiplication looks
like, and then cover an optimized version of the process.

### Simple Matrix Multiplication

The formal defitions for matrix multiplication can use somewhat terse
notation, so if you are unfamiliar it might be better to start with
an example

Let's say we have two matrices, `a` and `b`, such that:

```
a = [0, 1]    b = [0, 1, 2]
    [1, 2]        [2, 4, 8]
    [2  3]
```

In this case, `c = a * b` would look like:

```
3x2 matrix * 2x3 matrix = 3x3 matrix
c = [ (0*0 + 1*2), (0*1 + 1*4), (0*2 + 1*8) ]
    [ (1*0 + 2*2), (1*1 + 2*4), (1*2 + 2*8) ]
    [ (2*0 + 3*2), (2*1 + 3*4), (2*2 + 3*8) ]

c = [2, 4,  8 ]
    [4, 9,  18]
    [6, 14, 28]
```

The idea is that we are multiplying elements in the rows in `a` with the
columns in `b`, and summing these products to find the value in a cell of `c`.
An important caveat we will need to remember is the rule that we can only
multiply two matrices of dimensions `m x n` and `n x o`. In other words,
`a` must have a number of columns equal to the number of rows in `b`.

This will end up being a slow operation because of the large number of reads
involved. Each element in both `a` and `b` will need to be referenced a
number of times. We will cover the optimized version next.

### Fast Matrix Multiplication

To do ...

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

