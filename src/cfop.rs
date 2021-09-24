//! Collection of CFOP related tools.

use crate::matrix;
use crate::matrix::PermutationMatrix;
use crate::{Command, Move};

const U: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const D: [usize; 9] = [9, 10, 11, 12, 13, 14, 15, 16, 17];
const F: [usize; 9] = [18, 19, 20, 21, 22, 23, 24, 25, 26];
const B: [usize; 9] = [27, 28, 29, 30, 31, 32, 33, 34, 35];
const R: [usize; 9] = [36, 37, 38, 39, 40, 41, 42, 43, 44];
const L: [usize; 9] = [45, 46, 47, 48, 49, 50, 51, 52, 53];
const F2: [usize; 6] = [21, 22, 23, 24, 25, 26];
const B2: [usize; 6] = [27, 28, 30, 31, 33, 34];
const R2: [usize; 6] = [36, 37, 39, 40, 42, 43];
const L2: [usize; 6] = [48, 49, 50, 51, 52, 53];

/// Check if the cube is solved.
pub fn solved(mat: &PermutationMatrix) -> bool {
    matrix::same_color_check(mat, U)
        && matrix::same_color_check(mat, D)
        && matrix::same_color_check(mat, F)
        && matrix::same_color_check(mat, B)
        && matrix::same_color_check(mat, R)
        && matrix::same_color_check(mat, L)
}

/// Check if the F2L is solved.
/// This function assumes the layer is the down two layers.
pub fn f2l_solved(mat: &PermutationMatrix) -> bool {
    matrix::same_color_check(mat, U)
        && matrix::same_color_check(mat, D)
        && matrix::same_color_check(mat, F2)
        && matrix::same_color_check(mat, B2)
        && matrix::same_color_check(mat, R2)
        && matrix::same_color_check(mat, L2)
}

#[test]
fn test_solved() {
    let mut m = PermutationMatrix::identity();
    for mov in [Move::U, Move::D, Move::F, Move::B, Move::R, Move::L] {
        let c = matrix::of(Command(mov, 1));
        assert!(solved(&m));
        m = c * m;
        assert!(!solved(&m));
        m = c * m;
        assert!(!solved(&m));
        m = c * m;
        assert!(!solved(&m));
        m = c * m;
        assert!(solved(&m));
    }
}
#[test]
fn test_f2l_solved_u() {
    let mut m = PermutationMatrix::identity();
    let u = matrix::of(Command(Move::U, 1));
    assert!(f2l_solved(&m));
    m = u * m;
    assert!(f2l_solved(&m));
}
#[test]
fn test_f2l_solved_4times() {
    let mut m = PermutationMatrix::identity();
    for mov in [Move::D, Move::F, Move::B, Move::R, Move::L] {
        let c = matrix::of(Command(mov, 1));
        assert!(f2l_solved(&m));
        m = c * m;
        assert!(!f2l_solved(&m));
        m = c * m;
        assert!(!f2l_solved(&m));
        m = c * m;
        assert!(!f2l_solved(&m));
        m = c * m;
        assert!(f2l_solved(&m));
    }
}
