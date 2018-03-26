pub mod multiply_naive;
pub mod multiply_fast;

#[cfg(test)]
mod tests {
    use multiply_naive::hello_world;
    #[test]
    fn it_works() {
        let expected_res = "Hello World!";
        let actual_res = hello_world();
        assert_eq!(actual_res, expected_res);
    }
}
