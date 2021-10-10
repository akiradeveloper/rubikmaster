#![feature(test)]

extern crate test;

use rubikmaster as M;

#[bench]
fn bench_matmul(b: &mut test::Bencher) {
    let mut seq = vec![];
    for c in M::random(100) {
        seq.push(M::matrix::of(M::coord::rotation_of(c)));
    }
    b.iter(|| {
        let mut mat = M::matrix::PermutationMatrix::identity();
        for &x in &seq {
            mat = x * mat;
        }
    })
}
