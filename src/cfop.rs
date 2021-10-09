//! Collection of CFOP related tools.

use crate::matrix;
use crate::matrix::{same_color_check, PermutationMatrix};
use crate::{Command, Move};

const R: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const L: [u8; 9] = [9, 10, 11, 12, 13, 14, 15, 16, 17];
const U: [u8; 9] = [18, 19, 20, 21, 22, 23, 24, 25, 26];
const D: [u8; 9] = [27, 28, 29, 30, 31, 32, 33, 34, 35];
const F: [u8; 9] = [36, 37, 38, 39, 40, 41, 42, 43, 44];
const B: [u8; 9] = [45, 46, 47, 48, 49, 50, 51, 52, 53];
const R2: [u8; 6] = [0, 1, 3, 4, 6, 7];
const L2: [u8; 6] = [12, 13, 14, 15, 16, 17];
const F2: [u8; 6] = [39, 40, 41, 42, 43, 44];
const B2: [u8; 6] = [45, 46, 48, 49, 51, 52];

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
pub fn f2l_solved(mat: &PermutationMatrix) -> bool {
    same_color_check(mat, D)
        && same_color_check(mat, F2)
        && same_color_check(mat, B2)
        && same_color_check(mat, R2)
        && same_color_check(mat, L2)
}

/// Check if the OLL is solved.
pub fn oll_solved(mat: &PermutationMatrix) -> bool {
    same_color_check(mat, U) && f2l_solved(&mat)
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
    assert!(oll_solved(&m));
    let x = matrix::of(Command(Move::x, 1));
    m = x * m;
    assert!(!oll_solved(&m));
}
#[test]
fn test_f2l_solved_fx() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::F, 1));
    m = f * m;
    assert!(!oll_solved(&m));
    let x = matrix::of(Command(Move::x, 1));
    m = x * m;
    assert!(oll_solved(&m));
}
#[test]
fn test_f2l_solved_lz() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::L, 1));
    m = f * m;
    assert!(!oll_solved(&m));
    let x = matrix::of(Command(Move::z, 1));
    m = x * m;
    assert!(oll_solved(&m));
}
#[test]
fn test_f2l_solved_xu() {
    let mut m = PermutationMatrix::identity();
    let f = matrix::of(Command(Move::x, 1));
    m = f * m;
    assert!(oll_solved(&m));
    let x = matrix::of(Command(Move::U, 1));
    m = x * m;
    assert!(oll_solved(&m));
}
#[test]
fn test_f2l_solved_no_effect() {
    let mut m = PermutationMatrix::identity();
    for mov in [Move::U, Move::d, Move::x, Move::y, Move::z] {
        let u = matrix::of(Command(mov, 1));
        for _ in 0..1000 {
            m = u * m;
            assert!(oll_solved(&m));
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
        assert!(oll_solved(&m));
        m = c * m;
        assert!(!oll_solved(&m));
        m = c * m;
        assert!(!oll_solved(&m));
        m = c * m;
        assert!(!oll_solved(&m));
        m = c * m;
        assert!(oll_solved(&m));
    }
}

/// The list of F2L solutions. The problem numbers are from Tribox.
pub const F2L_LIST: [&str; 41] = [
    "URU'R'",                // 1
    "yU'L'UL",               // 2
    "yL'U'L",                // 3
    "RUR'",                  // 4
    "U'RUR'U2RU'R'",         // 5
    "yUL'U'LU2'L'UL",        // 6
    "U'RU2R'U2RU'R'",        // 7
    "yUL'U2LU2'L'UL",        // 8
    "yUL'U'LU'L'U'L",        // 9
    "U'RUR'URUR'",           // 10
    "U'RU2'R'Uy'R'U'R",      // 11
    "yUL'U2LU'yLUL'",        // 12
    "yUL'ULU'L'U'L",         // 13
    "U'RU'RURUR'",           // 14
    "yL'ULU2yLUL'",          // 15
    "RU'R'U2y'R'U'R",        // 16
    "RU2'R'U'RUR'",          // 17
    "yL'U2LUL'U'L",          // 18
    "URU2'R'URU'R'",         // 19
    "yU'L'U2LU'L'UL",        // 20
    "U2RUR'URU'R'",          // 21
    "yL'ULU2'L'U'L",         // 22
    "URU'RU'RU'R'URU'R'",    // 23
    "yU'L'ULUL'ULU'L'UL",    // 24
    "yU'L'ULUy'RU'R'",       // 25
    "URU'R'U'yL'UL",         // 26
    "RU'R'URU'R'",           // 27
    "yL'ULU'L'UL",           // 28
    "R'FRF'URU'R",           // 29
    "RUR'U'RUR'",            // 30
    "U'R'FRF'RU'R'",         // 31
    "RUR'U'RUR'U'RUR'",      // 32
    "U'RU'R'U2RU'R'",        // 33
    "yUL'ULU2'L'UL",         // 34
    "U'RUR'Uy'R'U'R",        // 35
    "yUL'U'LU'yLUL'",        // 36
    "RU2'R'URU2'R'Uy'R'U'R", // 37
    "RU'R'U'RUR'U2RU'R'",    // 38
    "RU'R'URU2'R'URU'R'",    // 39
    "RU'R'U'RU'R'Uy'R'U'R",  // 40
    "yL'ULUL'ULU'yLUL'",     // 41
];
#[test]
fn test_f2l_parse() {
    for seq in F2L_LIST {
        assert!(crate::parser::parse(&seq).is_ok());
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
        m = m.inv();
        assert!(!solved(&m));
        assert!(oll_solved(&m));
    }
}

pub const OLL_LIST: [&str; 57] = [
    "RU2R2FRF'U2R'FRF'",
    "rUr'U2rU2R'U2RU'r'",
    "r'R2UR'UrU2r'UM'",
    "MU'rU2r'U'RU'R'M'",
    "l'U2LUL'Ul",
    "rU2R'U'RU'r'",
    "rUR'URU2r'",
    "l'U'LU'L'U2l",
    "RUR'U'R'FR2UR'U'F'",
    "RUR'UR'FRF'RU2R'",
    "rUR'UR'FRF'RU2r'",
    "M'R'U'RU'R'U2RU'Rr'",
    "FURU'R2F'RURU'R'",
    "R'FRUR'F'RFU'F'",
    "l'U'lL'U'LUl'Ul",
    "rUr'RUR'U'rU'r'",
    "FR'F'R2r'URU'R'U'M'",
    "rUR'URU2r2U'RU'R'U2r",
    "r'RURUR'U'M'R'FRF'",
    "rUR'U'M2URU'R'U'M'",
    "yRU2R'U'RUR'U'RU'R'",
    "RU2R2U'R2U'R2U2R",
    "R2DR'U2RD'R'U2R'",
    "rUR'U'r'FRF'",
    "yF'rUR'U'r'FR",
    "yRU2R'U'RU'R'",
    "RUR'URU2R'",
    "rUR'U'MURU'R'",
    "MURUR'U'R'FRF'M'",
    "y2FURU2R'U'RU2R'U'F'",
    "R'U'FURU'R'F'R",
    "SRUR'U'R'FRf'",
    "RUR'U'R'FRF'",
    "y2RUR2U'R'FRURU'F'",
    "RU2R2FRF'RU2R'",
    "y2L'U'LU'L'ULULF'L'F",
    "FRU'R'U'RUR'F'",
    "RUR'URU'R'U'R'FRF'",
    "yLF'L'U'LUFU'L'",
    "yR'FRUR'U'F'UR",
    "y2RUR'URU2R'FRUR'U'F'",
    "R'U'RU'R'U2RFRUR'U'F'",
    "f'L'U'LUf",
    "fRUR'U'f'",
    "FRUR'U'F'",
    "R'U'R'FRF'UR",
    "F'L'U'LUL'U'LUF",
    "FRUR'U'RUR'U'F'",
    "y2rU'r2Ur2Ur2U'r",
    "r'Ur2U'r2U'r2Ur'",
    "fRUR'U'RUR'U'f'",
    "RUR'URd'RU'R'F'",
    "r'U'RU'R'URU'R'U2r",
    "rUR'URU'R'URU2r'",
    "RU2R2U'RU'R'U2FRF'",
    "rUr'URU'R'URU'R'rU'r'",
    "RUR'U'M'URU'r'",
];
#[test]
fn test_oll_list() {
    let mut no = 0;
    for seq in OLL_LIST {
        no += 1;
        println!("OLL-{}", no);
        let mut m = PermutationMatrix::identity();
        let elems = crate::parser::parse(&seq).unwrap().1;
        let cs = crate::flatten(elems);
        for c in cs {
            m = matrix::of(c) * m;
        }
        m = m.inv();
        assert!(!solved(&m));
        assert!(f2l_solved(&m));
    }
}
