use ndarray::Array2;

type Matrix<T> = Array2<T>;

pub fn hello_world() -> &'static str {
    return "Hello World!";
}

pub fn multiply<T>(a:&Matrix<T>, b:&Matrix<T>) -> Result<Matrix<T>, String> {
    unimplemented!()
}
