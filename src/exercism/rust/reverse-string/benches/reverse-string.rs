#![feature(test)]

extern crate test;

use reverse_string::*;

#[cfg(test)]
use test::{Bencher, black_box};

fn process_reverse_case(input: &str, expected: &str) {
    assert_eq!(&reverse(input), expected)
}

#[bench]
fn bench_pow(b: &mut Bencher) {
    b.iter(||
        //for i in 1..100 {
            black_box(process_reverse_case("Ramen", "nemaR"))
        //}
    )
}
