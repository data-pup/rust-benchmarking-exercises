#![feature(test)]
extern crate test;
use test::black_box;

extern crate ndarray;
use ndarray::Array;

extern crate rust_benchmarking_exercise;
use rust_benchmarking_exercise::multiply_naive::multiply;

#[bench]
fn naive_multiply_bench(bencher: &mut test::Bencher) {
    let a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros((64, 64));
    black_box(&a);
    black_box(&b);
    bencher.iter(|| {
        multiply(&a, &b)
    });
}

#[bench]
fn ndarray_multiply_bench(bencher: &mut test::Bencher) {
    let a = Array::<i32, _>::zeros((64, 64));
    let b = Array::<i32, _>::zeros((64, 64));
    black_box(&a);
    black_box(&b);
    bencher.iter(|| {
        a.dot(&b)
    });
}
