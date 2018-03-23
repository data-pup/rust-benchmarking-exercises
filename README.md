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

For the sake of benchmarking, we will compare runtime of some common operations
on different data structures. This will also function as a convenient excuse
to build some neat data structures.

### What is a heap?

A binary heap is a binary tree implemented such that the tree satifies the
heap property. In short, that for the whole tree, the value at each node is
(>= or <=) than the values stored at each of its child nodes. Consequently,
either the minimum or maximum value in the heap is always found at the top.

To gloss over everything for now, a binary heap also seems like an intriguing
enough problem to force me to get more understanding concerning how to
build things using Rust's borrowing and ownership systems effectively.

### Builing a heap:

WIP ...

## Lessons, Discoveries

...

### Verify Project Correctness

`cargo verify-project` will check the correctness of the crate manifest.
If the project passed the check, you will see a result printed to output
like so:

```sh
✨  cargo verify-project
{"success":"true"}
```

