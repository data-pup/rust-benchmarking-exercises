extern crate rust_benchmarking_exercise;

use rust_benchmarking_exercise::{multiply_fast, multiply_naive};
use multiply_naive::hello_world as naive_hello;
use multiply_fast::hello_world as fast_hello;

#[test]
fn naive_hello_works() {
    let expected_res = "Hello World!";
    let actual_res = naive_hello();
    assert_eq!(actual_res, expected_res);
}

#[test]
fn fast_hello_works() {
    let expected_res = "Hello World!";
    let actual_res = fast_hello();
    assert_eq!(actual_res, expected_res);
}
