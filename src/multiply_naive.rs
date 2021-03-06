use multiply_utils::{Matrix, MatrixPosition};
use multiply_utils::init_output_matrix;
use num_traits::{Num, NumOps};
use std::clone::Clone;

/// Multiply two matrices of numeric values, and return the result.
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

/// Calculate the value of a cell in the output matrix, given the position
/// coordinates (i, j). Returns an error if the slices of `a` and `b` were
/// unexpectedly not of the same length.
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
