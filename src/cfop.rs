//! Collection of CFOP related tools.

use crate::matrix;
use crate::matrix::{same_color_check, PermutationMatrix};
use crate::{Command, Move};

const R: [usize; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const L: [usize; 9] = [9, 10, 11, 12, 13, 14, 15, 16, 17];
const U: [usize; 9] = [18, 19, 20, 21, 22, 23, 24, 25, 26];
const D: [usize; 9] = [27, 28, 29, 30, 31, 32, 33, 34, 35];
const F: [usize; 9] = [36, 37, 38, 39, 40, 41, 42, 43, 44];
const B: [usize; 9] = [45, 46, 47, 48, 49, 50, 51, 52, 53];
const R2: [usize; 6] = [0, 1, 3, 4, 6, 7];
const L2: [usize; 6] = [12, 13, 14, 15, 16, 17];
const F2: [usize; 6] = [39, 40, 41, 42, 43, 44];
const B2: [usize; 6] = [45, 46, 48, 49, 51, 52];

/// Check if the cube is solved.
pub fn solved(mat: &PermutationMatrix) -> bool {
    same_color_check(mat, U)
        && same_color_check(mat, D)
        && same_color_check(mat, F)
        && same_color_check(mat, B)
        && same_color_check(mat, R)
        && same_color_check(mat, L)
}

/// Check if the F2L is solved.
/// This function assumes the layer is the down two layers.
pub fn f2l_solved(mat: &PermutationMatrix) -> bool {
    same_color_check(mat, U)
        && same_color_check(mat, D)
        && same_color_check(mat, F2)
        && same_color_check(mat, B2)
        && same_color_check(mat, R2)
        && same_color_check(mat, L2)
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

pub const PLL_LIST: [(&str, &str); 2] = [
    ("Ub", "R2URUR'U'R'U'R'UR'"),
    ("Ua", "RU'RURURU'R'U'R2"),
    // TODO
];
#[test]
fn test_pll_list() {
    for (_, seq) in PLL_LIST {
        let mut m = PermutationMatrix::identity();
        let elems = crate::parser::parse(&seq).unwrap().1;
        let cs = crate::flatten(elems);
        for c in cs {
            m = matrix::of(c) * m;
        }
        assert!(!solved(&m) && f2l_solved(&m));
    }
}
