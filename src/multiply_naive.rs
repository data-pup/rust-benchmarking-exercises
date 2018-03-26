use ndarray::Array2;

type Matrix<T> = Array2<T>;

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String> {
    unimplemented!()
}

mod tests {
    use multiply_naive::multiply;
    #[test]
    fn it_works() {
        let a = array![[0, 1],
                       [2, 3]];
        let b = array![[0, 1],
                       [2, 3]];
        let c = multiply(&a, &b);
    }
}
