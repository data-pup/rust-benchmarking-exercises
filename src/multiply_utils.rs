use ndarray::prelude::*;
use num_traits::Num;

pub type Matrix<T> = Array2<T>;
pub type MatrixDimensions = (usize, usize);
pub type MatrixPosition = (usize, usize);

/// Initialize the output matrix. If there was a dimension problem with the
/// inputs, returns an Err. Otherwise, a new matrix filled with zeros is created.
pub fn init_output_matrix<T>(a:&Matrix<T>, b:&Matrix<T>)
    -> Result<Matrix<T>, String>
    where T: Clone + Num
{
    let (a_dims, b_dims) = (a.dim(), b.dim());
    let dimensions:MatrixDimensions = get_output_dims(a_dims, b_dims)?;
    let m:Matrix<T> = Array::<T, _>::zeros(dimensions);
    Ok(m)
}

/// Get the dimensions of the output matrix, given the dimensions of `a` and `b`.
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
    use multiply_utils::*;
    use self::test_cases::*;

    #[test]
    fn mismatched_dims_causes_error() {
        for curr_case in MISMATCHED_DIMENSIONS.iter() {
            let &InvalidDimensionsTestCase {a_dims, b_dims} = curr_case;
            let res = get_output_dims(a_dims, b_dims);
            assert!(res.is_err(), "Invalid dimensions should not be accepted!");
        }
    }

    #[test]
    fn output_dimensions_are_correct() {
        for curr_case in VALID_DIMENSIONS_TESTS.iter() {
            let &ValidDimensionsTestCase {
                a_dims, b_dims,
                expected_c_dims:(expected_c_height, expected_c_width)
            } = curr_case;
            let (actual_c_height, actual_c_width):MatrixDimensions =
                get_output_dims(a_dims, b_dims).unwrap();
            assert_eq!(actual_c_height, expected_c_height);
            assert_eq!(actual_c_width, expected_c_width);
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_cases {
    use multiply_utils::{MatrixDimensions};

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

    pub struct InvalidDimensionsTestCase {
        pub a_dims:MatrixDimensions,
        pub b_dims:MatrixDimensions,
    }

    pub static MISMATCHED_DIMENSIONS:[InvalidDimensionsTestCase; 2] = [
        InvalidDimensionsTestCase { a_dims:(1, 2), b_dims:(1, 2) },
        InvalidDimensionsTestCase { a_dims:(2, 1), b_dims:(2, 2) },
    ];
}
