#![feature(test)]
extern crate test;

use envmnt;
use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let exists = envmnt::exists("BENCH_EXIST_ENV");
        assert!(!exists);
    });
}
