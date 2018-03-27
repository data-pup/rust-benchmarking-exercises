use multiply_utils::{Matrix, MatrixPosition};
use multiply_utils::init_output_matrix;
use num_traits::{Num, NumOps};
use std::clone::Clone;

/// Temporary function used while scaffolding.
pub fn hello_world() -> &'static str {
    return "Hello World!";
}

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

#[cfg(test)]
mod tests {
    use ndarray::Array;
    use multiply_naive::*;
    use self::test_cases::*;

    #[test]
    fn multiply_works() {
        for curr_case in MULTIPLY_TESTS.iter() {
            let (a, b, expected_c) = initialize_test_matrices(curr_case);
            let actual_c = multiply(&a, &b).unwrap();
            assert_eq!(actual_c, expected_c,
                "Test Failed: {}", curr_case.desc);
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_cases {
    use multiply_utils::{Matrix, MatrixDimensions};
    use ndarray::Array;
    use num_traits::{Num, NumOps};
    use std::clone::Clone;

    /// This structure defines a multiplication test case, including the
    /// dimensions and elements for the inputs of a test case, as well as
    /// those of the expected output.
    pub struct MultiplyTest<T: 'static>
        where T: Clone + Num + NumOps
    {
        pub desc:&'static str,
        pub a_dims:MatrixDimensions, pub a_elems:&'static [T],
        pub b_dims:MatrixDimensions, pub b_elems:&'static [T],
        pub c_dims:MatrixDimensions, pub c_elems:&'static [T],
    }

    /// Define the test cases to run.
    pub static MULTIPLY_TESTS:[MultiplyTest<u32>; 2] = [
        MultiplyTest {
            desc:"Multiply two 1x1 matrices",
            a_dims:(1, 1),
            a_elems:&[2],
            b_dims:(1, 1),
            b_elems:&[2],
            c_dims:(1, 1),
            c_elems:&[4],
        },
        MultiplyTest {
            desc:"Multiply a 3x2 matrix with a 2x3 matrix",
            a_dims:(3, 2),
            a_elems:&[
                1, 0,
                0, 1,
                1, 1,
            ],
            b_dims:(2, 3),
            b_elems:&[
                0, 1, 2,
                2, 4, 8,
            ],
            c_dims:(3, 3),
            c_elems:&[
                0, 1, 2,
                2, 4, 8,
                2, 5, 10,
            ]
        },
    ];

    /// Initializes the input/result matrices for a test case.
    pub fn initialize_test_matrices<T>(test_case:&MultiplyTest<T>)
        -> (Matrix<T>, Matrix<T>, Matrix<T>)
        where T: Clone + Num + NumOps
    {
        let &MultiplyTest {
            a_dims, a_elems, b_dims, b_elems, c_dims, c_elems, desc:_desc
        } = test_case;
        return (
            init_test_matrix(a_dims, a_elems),
            init_test_matrix(b_dims, b_elems),
            init_test_matrix(c_dims, c_elems),
        );
    }

    /// Initialize a single test matrix.
    fn init_test_matrix<T: 'static>(dims:MatrixDimensions, elems:&[T])
        -> Matrix<T>
        where T: Clone + Num + NumOps
    {
        return Array::from_shape_vec(dims, elems.to_vec()).unwrap();
    }
}
