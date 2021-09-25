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
fn test_solved_no_effect() {
    let mut m = PermutationMatrix::identity();
    for mov in [Move::x, Move::y, Move::z] {
        let u = matrix::of(Command(mov, 1));
        for _ in 0..1000 {
            m = u * m;
            assert!(solved(&m));
        }
    }
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
fn test_f2l_solved_ux() {
    let mut m = PermutationMatrix::identity();
    let u = matrix::of(Command(Move::U, 1));
    m = u * m;
    assert!(f2l_solved(&m));
    let x = matrix::of(Command(Move::x, 1));
    m = x * m;
    assert!(!f2l_solved(&m));
}
#[test]
fn test_f2l_solved_fx() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::F, 1));
    m = f * m;
    assert!(!f2l_solved(&m));
    let x = matrix::of(Command(Move::x, 1));
    m = x * m;
    assert!(f2l_solved(&m));
}
#[test]
fn test_f2l_solved_lz() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::L, 1));
    m = f * m;
    assert!(!f2l_solved(&m));
    let x = matrix::of(Command(Move::z, 1));
    m = x * m;
    assert!(f2l_solved(&m));
}
#[test]
fn test_f2l_solved_xu() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::x, 1));
    m = f * m;
    assert!(f2l_solved(&m));
    let x = matrix::of(Command(Move::U, 1));
    m = x * m;
    assert!(f2l_solved(&m));
}
#[test]
fn test_f2l_solved_no_effect() {
    let mut m = PermutationMatrix::identity();
    for mov in [Move::U, Move::d, Move::x, Move::y, Move::z] {
        let u = matrix::of(Command(mov, 1));
        for _ in 0..1000 {
            m = u * m;
            assert!(f2l_solved(&m));
        }
    }
}
#[test]
fn test_f2l_solved_4times() {
    let mut m = PermutationMatrix::identity();
    for mov in [
        Move::D,
        Move::F,
        Move::B,
        Move::R,
        Move::L,
        Move::u,
        Move::f,
        Move::b,
        Move::r,
        Move::l,
    ] {
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

pub const PLL_LIST: [(&str, &str); 21] = [
    ("Ub", "(R2'U)(RUR')(U'R'U')(R'UR')"),
    ("Ua", "(RU'R)(URUR)(U'R'U'R2')"),
    ("Ab", "xR2'(D2RUR')(D2RU'R)x'"),
    ("Aa", "x(R'UR'D2)(RU'R'D2)R2x'"),
    ("Z", "(M2UM2U)M'(U2M2U2)M'"),
    ("H", "M2UM2U2M2UM2"),
    ("E", "x'(RU'R'D)(RUR'D')(RUR'D)(RU'R'D')x"),
    ("T", "(RUR'U')(R'FR2U'R'U')(RUR'F')"),
    ("V", "(R'UR'd')(R'F'R2U')(R'UR')(FRF)"),
    ("F", "(R'U'F')(RUR'U')(R'FR2U'R'U')(RUR'UR)"),
    ("Rb", "(R'U2RU2)(R'FRUR'U')(R'F'R2'U')"),
    ("Ra", "(RU'R'U')(RURD)(R'U'RD')(R'U2R')"),
    ("Jb", "(RUR'F')(RUR'U')(R'FR2U'R'U')"),
    ("Ja", "(UR'U)(L'U2RU'R'U2)RL"),
    ("Y", "(FRU'R'U')(RUR'F')(RUR'U')(R'FRF')"),
    ("Gd", "D'(RUR')U'D(R2U'RU')(R'UR'UR2)"),
    ("Gc", "(R2U'RU')(RUR'UR2)UD'(RU'R'D)"),
    ("Ga", "R2u(R'UR'U'R)u'R2y'(R'UR)"),
    ("Gb", "(R'U'R)yR2u(R'URU'R)u'R2"),
    ("Nb", "(R'URU')(R'F'U')(FRUR'F)(R'F'RU'R)"),
    ("Na", "(RUR'U)(RUR'F')(RUR'U')(R'FR2U'R'U2'RU'R')"),
];
#[test]
fn test_pll_list() {
    for (perm, seq) in PLL_LIST {
        println!("perm={}", perm);
        let mut m = PermutationMatrix::identity();
        let elems = crate::parser::parse(&seq).unwrap().1;
        let cs = crate::flatten(elems);
        for c in cs {
            m = matrix::of(c) * m;
        }
        assert!(!solved(&m));
        assert!(f2l_solved(&m));
    }
}
