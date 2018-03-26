use ndarray::Array2;

type Matrix<T> = Array2<T>;

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String> {
    let (m_a, n_a) = a.dim();
    let (n_b, o_b) = b.dim();
    let (c_height, c_width) = match n_a == n_b {
        true => (m_a, o_b),
        false => return Err("Incorrect matrix dimensions given!".to_owned()),
    };

    unimplemented!()
}

mod tests {
    use ndarray::prelude::*;
    use multiply_naive::multiply;

    #[test]
    fn mismatched_dims_causes_error() {
        let dims = (2, 3);
        let a = Array::<u32, _>::zeros(dims);
        let b = Array::<u32, _>::zeros(dims);
        let res = multiply(&a, &b);
        assert!(res.is_err(), "Invalid dimensions should not be accepted!");
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
