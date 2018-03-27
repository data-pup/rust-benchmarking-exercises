use ndarray::prelude::*;
use num_traits::{Num, NumOps};
use multiply_utils::{Matrix, MatrixDimensions};
use std::clone::Clone;

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String>
    where T: Clone + Num + NumOps
{
    unimplemented!();

}

// Initializes an output matrix for matrix multiplication following the
// Strassen algorithm. If a or b are not square matrices
fn init_strassen_output_matrix<T>(a:&Matrix<T>, b:&Matrix<T>)
    -> Result<Matrix<T>, String>
    where T: Clone + Num
{
    if !a.is_square() || !b.is_square() {
        return Err("Strassen multiplication requires square inputs!".to_owned());
    }

    let size:usize = match a.rows() == b.rows() {
        true => a.rows(),
        false => return Err("Input matrices must be of same size!".to_owned()),
    };

    let dimensions:MatrixDimensions = (size, size);
    let m:Matrix<T> = Array::<T, _>::zeros(dimensions);
    Ok(m)
}
