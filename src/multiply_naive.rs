use ndarray::Array;
use ndarray::Array2;

use num_traits::Num;
use std::clone::Clone;

type Matrix<T> = Array2<T>;

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String>
    where T: Clone + Num
{
    let c_dims = get_output_dims(&a, &b)?;
    let mut c = Array::<T, _>::zeros(c_dims);
    unimplemented!()
}

fn get_output_dims<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<(usize, usize), String> {
    let (m_a, n_a) = a.dim();
    let (n_b, o_b) = b.dim();
    match n_a == n_b {
        true => Ok((m_a, o_b)),
        false => Err("Incorrect matrix dimensions given!".to_owned()),
    }
}

#[cfg(test)]
mod tests {
    use ndarray::Array;
    use multiply_naive::multiply;
    use multiply_naive::test_cases;

    #[test]
    fn mismatched_dims_causes_error() {
        for curr_case in test_cases::MISMATCHED_DIMENSIONS.iter() {
            let &(a_dims, b_dims) = curr_case;
            let a = Array::<u32, _>::zeros(a_dims);
            let b = Array::<u32, _>::zeros(b_dims);
            let res = multiply(&a, &b);
            assert!(res.is_err(), "Invalid dimensions should not be accepted!");
        }
    }

    #[test]
    fn it_works() {
        let a = array![[0, 1],
                       [2, 3]];
        let b = array![[0, 1],
                       [2, 3]];
        let c = multiply(&a, &b);
    }
}

#[allow(dead_code)]
mod test_cases {
    type DimensionPair = ((usize, usize), (usize, usize));
    pub static MISMATCHED_DIMENSIONS:[DimensionPair; 2] = [
        ((1, 2), (1, 2)),
        ((2, 1), (2, 2)),
    ];
}
