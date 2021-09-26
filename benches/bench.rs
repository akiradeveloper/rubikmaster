#![feature(test)]

extern crate test;

use rubikmaster as M;

#[bench]
fn bench_matmul_100000(b: &mut test::Bencher) {
    let mut seq = vec![];
    for c in M::random(100_000) {
        seq.push(c);
    }
    b.iter(|| {
        let mut mat = M::matrix::PermutationMatrix::identity();
        for &x in &seq {
            mat = M::matrix::of(x) * mat;
        }
    })
}
#[bench]
fn bench_matmul_100000_par(b: &mut test::Bencher) {
    let mut seq = vec![];
    for c in M::random(100_000) {
        seq.push(c);
    }
    b.iter(|| {
        let mat = M::matrix::reduce(seq.clone());
    })
}
#[bench]
fn bench_matmul_10(b: &mut test::Bencher) {
    let mut seq = vec![];
    for c in M::random(10) {
        seq.push(c);
    }
    b.iter(|| {
        let mut mat = M::matrix::PermutationMatrix::identity();
        for &x in &seq {
            mat = M::matrix::of(x) * mat;
        }
    })
}
#[bench]
fn bench_matmul_10_par(b: &mut test::Bencher) {
    let mut seq = vec![];
    for c in M::random(10) {
        seq.push(c);
    }
    b.iter(|| {
        let mat = M::matrix::reduce(seq.clone());
    })
}