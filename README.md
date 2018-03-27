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

## Naive Implementation of Matrix Multiplication

### Overview

Before we go any further, let's cover what simple matrix multiplication looks
like, and then cover an optimized version of the process.

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

### Implementation

To build this, I first started by writing the logic used to find the dimensions
of the output matrix. I used the `ndarray` crate in my project, which provides
some useful classes to represent various n-dimensional arrays generically.
These objects provide a `.dim()` method, which returns a tuple of `usize` values
that represent the size of the matrix.

Using this, I implemented a function to first check the inputs' dimensions:

```rust
pub type MatrixDimensions = (usize, usize);

fn get_output_dims(a:MatrixDimensions, b:MatrixDimensions)
    -> Result<MatrixDimensions, String>
{
    let (a_height, a_width) = a;
    let (b_height, b_width) = b;
    match a_width == b_height {
        true => Ok((a_height, b_width)),
        false => Err("Incorrect matrix dimensions given!".to_owned()),
    }
}
```

This is used by a function that then initializes an output matrix filled with
zeros, assuming this function returned an `Ok` result.

```rust
pub type Matrix<T> = Array2<T>;
pub type MatrixDimensions = (usize, usize);

pub fn init_output_matrix<T>(a:&Matrix<T>, b:&Matrix<T>)
    -> Result<Matrix<T>, String>
    where T: Clone + Num
{
    let (a_dims, b_dims) = (a.dim(), b.dim());
    let dimensions:MatrixDimensions = get_output_dims(a_dims, b_dims)?;
    let m:Matrix<T> = Array::<T, _>::zeros(dimensions);
    return Ok(m);
}

```

Once I had initialized an output matrix, I just needed to calculate the value
of each cell in the matrix, using its position `(i, j)` to determine the
corresponding slices in `a` and `b`. The code to determine a cell value looked
like this:

```rust
fn get_curr_cell_value<T>((i, j):MatrixPosition, a:&Matrix<T>, b:&Matrix<T>)
    -> Result<T, String>
    where T: Clone + Num + NumOps
{
    let (a_row, b_col) = (a.slice(s![i, ..]), b.slice(s![.., j]));
    let elem_count = match a_row.len() == b_col.len() {
        true => a_row.len(),
        false => return Err("Dimension error while multiplying slices.".to_owned()),
    };
    let result = (0..elem_count)
        .map(|index| a_row[index].clone() * b_col[index].clone())
        .fold(T::zero(), |acc, elem| acc + elem);
    Ok(result)
}
```

This is used by the public `multiply(..)` method, which assigns a value to each
position in the output like so:

```rust
pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String>
    where T: Clone + Num + NumOps
{
    let mut c:Matrix<T> = init_output_matrix(&a, &b)?;
    for i in 0..c.rows() {
        for j in 0..c.cols() {
            c[[i, j]] = get_curr_cell_value((i, j), a, b)?;
        }
    }
    Ok(c)
}
```

There are some implementation details that could look different, and could
certainly be optimized further, but this is a learning project after all, and
we can give things another go with the next implementation we are tackling.
In the next section, we will build an optimized matrix multiplication algorithm.

## The Strassen Algorithm

We will next implement the Strassen algorithm, and benchmark this implementation
against the naive implementation that we build in the previous section.

### Overview

The Strassen algorithm takes advantage of the fact that it is possible to
represent the product of two 2x2 matrices using only 7 multiplication
operations rather than the 8 needed if performing the operation naively.

If we have input matrices A, B, such that:

```
A = [A_11, A_12]    B = [B_11, B_12]
    [A_21, A_22]        [B_21, B_22]
```

Then the naive method to calculate C would look like this:

```
C = [A_11*B_11 + A_12*B_21,  A_11*B_12 + A_12*B_22]
    [A_21*B_11 + A_22*B_21,  A_21*B_12 + A_22*B_22]
```

Instead, we can define 7 new matrices using the inputs:

```
M_1 = (A_11 + A_22) * (B_11 + B_22)
M_2 = (A_21 + A_22) * B_11
M_3 = A_11 * (B_12 - B_22)
M_4 = A_22 * (B_21 - B_11)
M_5 = (A_11 + A_12) * B_22
M_6 = (A_21 - A_11) * (B_11 + B_12)
M_7 = (A_12 - A_22) * (B_21 + B_22)
```

Using these matrices, we can represent C like this:

```
C_11 = M_1 + M_4 - M_5 + M_7
C_12 = M_3 + M_5
C_21 = M_2 + M_4
C_22 = M_1 - M_2 + M_3 + M_6
```

Note that only 7 multiplication operations are needed to generate the `M_n`
matrices, and these are only used in addition and subtraction operations when
calculating the results in C.

This process is performed recursively on the arrays, which means that we require
the matrices A and B to be square, and have height and width equal to a power
of two. If the input matrices do not match these conditions, they will be
padded with zeros.

Technically, this algorithm can be implemented to avoid these requirements, but
for simplicity we will follow the padding rule.

## Lessons, Discoveries

### The ? Operator

When dealing with `Result<T, E>` objects in a function that also returns a
`Result`, match statements that look like this become very common.

```rust
let foo = match do_something() {
    Ok(res) => result
    Err(err) => return Err(err);
};
```

We can accomplish the same task using the `?` operator. This helps keep code
concise without sacrificing the ability to correctly handle an error, passing
it up to the original caller. The code above could instead be written as:

```rust
let foo = do_something()?;
```

### Crates Ecosystem

For matrix multiplication, I found a nice crate online that offered
n-dimensional matrices. One especially bright spot in the JavaScript
ecosystem for me is the robust dependency management tooling. Having
this ability in a systems language like Rust is really neat, and pulling in
extra package dependencies was very straightfoward.

One neat feature that I enjoyed discovering was the ability to enable macros
provided by an external crate, using the `#[macro_use]` attribute.

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

