#![feature(test)]
extern crate ci_info;
extern crate test;

use test::Bencher;

#[bench]
fn get(bencher: &mut Bencher) {
    bencher.iter(|| {
        let exists = envmnt::exists("BENCH_EXIST_ENV");
        assert!(!exists);
    });
}
