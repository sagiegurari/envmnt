#![feature(test)]
extern crate envmnt;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let exists = envmnt::exists("BENCH_EXIST_ENV");
        assert!(!exists);
    });
}
