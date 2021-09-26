#![feature(portable_simd)]

pub mod cfop;
pub mod coord;
pub mod matrix;
pub mod parser;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Move {
    R,
    L,
    U,
    D,
    F,
    B,
    r,
    l,
    u,
    d,
    f,
    b,
    M,
    E,
    S,
    x,
    y,
    z,
}
pub const MOVE_LIST: [Move; 18] = [
    Move::R,
    Move::L,
    Move::U,
    Move::D,
    Move::F,
    Move::B,
    Move::r,
    Move::l,
    Move::u,
    Move::d,
    Move::f,
    Move::b,
    Move::M,
    Move::E,
    Move::S,
    Move::x,
    Move::y,
    Move::z,
];

/// Representation of Cube Notation.
///
/// Example:
/// - R2 is represented as a pair of ratation and repeatance (R,2).
/// - R' is represented as (R,-1).
///
/// Repeatance should be non-zero and [-2,2].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command(pub Move, pub i8);
impl Command {
    pub fn prime(self) -> Self {
        Command(self.0, -self.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    One(Command),
    Group(Vec<Command>, i8),
}
/// Turn a move sequence into a sequence without parentheses.
pub fn flatten(elems: Vec<Elem>) -> Vec<Command> {
    let mut v = vec![];
    for e in elems {
        match e {
            Elem::One(c) => v.push(c),
            Elem::Group(cs, rep) => {
                if rep > 0 {
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c);
                        }
                    }
                } else {
                    let rep = -rep;
                    let mut cs = cs;
                    cs.reverse();
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c.prime())
                        }
                    }
                }
            }
        }
    }
    v
}
#[test]
fn test_flatten() {
    let e = Elem::Group(vec![Command(Move::U, 1), Command(Move::R, 1)], -1);
    let f = flatten(vec![e]);
    assert_eq!(f, vec![Command(Move::R, -1), Command(Move::U, -1)]);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Surface {
    R,
    L,
    U,
    D,
    F,
    B,
}
pub const SURFACE_LIST: [Surface; 6] = [
    Surface::R,
    Surface::L,
    Surface::U,
    Surface::D,
    Surface::F,
    Surface::B,
];

/// Generate a scramble sequence.
pub fn random(n: usize) -> Vec<Command> {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..n {
        let mov: usize = rng.gen();
        let mov = MOVE_LIST[mov % 18];
        let rep: usize = rng.gen();
        let rep = rep % 3 + 1;
        v.push(Command(mov, rep as i8));
    }
    v
}
