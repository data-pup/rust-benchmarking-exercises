use ndarray::prelude::*;
use num_traits::{Num, NumOps};

use std::clone::Clone;

type Matrix<T> = Array2<T>;
type MatrixDimensions = (usize, usize);

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String>
    where T: Clone + Num + NumOps
{
    let mut c:Matrix<T> = init_output_matrix(&a, &b)?;
    for i in 0..c.rows() {
        for j in 0..c.cols() {
            c[[i, j]] = get_curr_cell_value((i, i), a, b)?;
        }
    }
    Ok(c)
}

pub fn get_curr_cell_value<T>(c_pos:(usize, usize),
                              a:&Matrix<T>, b:&Matrix<T>) -> Result<T, String>
    where T: Clone + Num + NumOps
{
    let (i, j) = c_pos;
    let a_row = a.slice(s![i, ..]);
    let b_col = b.slice(s![.., j]);
    let elem_count = match a_row.len() == b_col.len() {
        true => a_row.len(),
        false => return Err(
            "Dimension error while multiplying slices.".to_owned()),
    };
    let result = (0..elem_count)
        .map(|index| a_row[index].clone() * b_col[index].clone())
        .fold(T::zero(), |acc, elem| acc + elem);
    Ok(result)
}

fn init_output_matrix<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String>
    where T: Clone + Num
{
    let (a_dims, b_dims) = (a.dim(), b.dim());
    let dimensions:MatrixDimensions = get_output_dims(a_dims, b_dims)?;
    let m:Matrix<T> = Array::<T, _>::zeros(dimensions);
    return Ok(m);
}

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

#[cfg(test)]
mod tests {
    use ndarray::Array;
    use multiply_naive::*;

    #[test]
    fn mismatched_dims_causes_error() {
        for curr_case in test_cases::MISMATCHED_DIMENSIONS.iter() {
            let &(a_dims, b_dims) = curr_case;
            let res = get_output_dims(a_dims, b_dims);
            assert!(res.is_err(), "Invalid dimensions should not be accepted!");
        }
    }

    #[test]
    fn output_dimensions_are_correct() {
        for curr_case in test_cases::VALID_DIMENSIONS_TESTS.iter() {
            let &test_cases::ValidDimensionsTestCase {
                a_dims, b_dims,
                expected_c_dims:(expected_c_height, expected_c_width)
            } = curr_case;
            let (actual_c_height, actual_c_width):MatrixDimensions =
                get_output_dims(a_dims, b_dims).unwrap();
            assert_eq!(actual_c_height, expected_c_height);
            assert_eq!(actual_c_width, expected_c_width);
        }
    }

    #[test]
    fn multiply_test() {
        let a = array![
            [0, 1],
            [1, 2],
            [2, 3],
        ];
        let b = array![
            [0, 1, 2],
            [2, 4, 8],
        ];
        let expected_c = array![
            [2, 4, 8 ],
            [4, 9, 18],
            [6, 14, 28],
        ];
        let actual_c = multiply(&a, &b).unwrap();
        assert_eq!(actual_c, expected_c);
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_cases {
    use multiply_naive::{Matrix, MatrixDimensions};

    pub struct ValidDimensionsTestCase {
        pub a_dims:MatrixDimensions,
        pub b_dims:MatrixDimensions,
        pub expected_c_dims:MatrixDimensions,
    }

    pub static VALID_DIMENSIONS_TESTS:[ValidDimensionsTestCase; 3] = [
        ValidDimensionsTestCase {
            a_dims:(1, 4), b_dims:(4, 1),
            expected_c_dims:(1, 1)
        },
        ValidDimensionsTestCase {
            a_dims:(2, 4), b_dims:(4, 1),
            expected_c_dims:(2, 1)
        },
        ValidDimensionsTestCase {
            a_dims:(1, 4), b_dims:(4, 2),
            expected_c_dims:(1, 2)
        },
    ];

    pub static MISMATCHED_DIMENSIONS:[(MatrixDimensions, MatrixDimensions); 2] = [
        ((1, 2), (1, 2)),
        ((2, 1), (2, 2)),
    ];

    pub struct MultiplyTestCase<T> {
        pub a:Matrix<T>,
        pub b:Matrix<T>,
        pub expected_c:Matrix<T>,
    }

    // TODO: Fixup, the array! macro does not work within static variables.
    // pub static MULTIPLY_TESTS:[MultiplyTestCase<u32>; 1] = [
    //     MultiplyTestCase {
    //         a: array![
    //             [0, 1],
    //             [1, 2],
    //             [2, 3],
    //         ],
    //         b: array![
    //             [0, 1, 2],
    //             [2, 4, 8],
    //         ],
    //         expected_c: array![
    //             [2, 4, 8 ],
    //             [4, 9, 18],
    //             [6, 14, 28],
    //         ],
    //     }
    // ];
}
