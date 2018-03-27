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

    let size_log2 = (size as f64).log2();
    let size_is_pow_of_2 = size_log2 == size_log2.floor();
    match size_is_pow_of_2 {
        true => Ok(Array::<T, _>::zeros((size, size))),
        false => Err("Dimensions of inputs must be a power of 2!".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use multiply_fast::*;
    use self::test_cases::*;

    #[test]
    fn invalid_dims_are_not_accepted() {
        for curr_test in INVALID_DIMENSIONS.iter() {
            let &InvalidDimensionsTestCase {desc, a_dims, b_dims} = curr_test;
            let a = Array::<u32, _>::zeros(a_dims);
            let b = Array::<u32, _>::zeros(b_dims);
            let c_res = init_strassen_output_matrix(&a, &b);
            assert!(c_res.is_err(), "Test Failed: {}", desc);
        }
    }
}

#[allow(dead_code)]
#[cfg(test)]
mod test_cases {
    use multiply_utils::{MatrixDimensions};

    pub struct InvalidDimensionsTestCase {
        pub desc:&'static str,
        pub a_dims:MatrixDimensions,
        pub b_dims:MatrixDimensions,
    }

    pub static INVALID_DIMENSIONS:[InvalidDimensionsTestCase; 3] = [
        InvalidDimensionsTestCase {
            desc:"Non-square matrices eligible for standard algorithm",
            a_dims:(1, 2), b_dims:(2, 1),
        },
        InvalidDimensionsTestCase {
            desc:"Mismatched dimensions should not be accepted",
            a_dims:(2, 1), b_dims:(2, 2),
        },
        InvalidDimensionsTestCase {
            desc:"Matching square matrices must be of size 2^n",
            a_dims:(9, 9), b_dims:(9, 9),
        },
    ];
}
