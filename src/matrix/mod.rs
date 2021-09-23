//! Cube's state is expressed as permutation matrix
//! and operations are matrix multiplications.

use crate::Command;
use crate::{Move, Surface, MOVE, SURFACE};
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod math;
pub use math::{Permutation, PermutationMatrix};

/// Conversion function from position in a surface to
/// a index in permutation.
pub fn pos(surface: Surface, i: usize, j: usize) -> usize {
    let n = surface as usize;
    9 * n + 3 * i + j
}
/// Inverse function of `pos`.
pub fn pos_inv(mut k: usize) -> (Surface, usize, usize) {
    let n = k / 9;
    k -= 9 * n;
    let i = k / 3;
    k -= 3 * i;
    let j = k;
    (SURFACE[n], i, j)
}
#[test]
fn test_pos() {
    assert_eq!(pos(Surface::F, 2, 2), 26);
    assert_eq!(pos_inv(26), (Surface::F, 2, 2));
}

/// Check if the colors on the given `positions` are the same.
pub fn same_color_check<const N: usize>(mat: &PermutationMatrix, positions: [usize; N]) -> bool {
    let inv = mat.inv_perm;
    let mut color_list = [Surface::B; N];
    for i in 0..N {
        let pos = inv[positions[i]];
        let (c, _, _) = pos_inv(pos);
        color_list[i] = c;
    }
    let mut b = true;
    for i in 0..N {
        b &= color_list[i] == color_list[(i + 1) % N];
    }
    b
}
struct Arrow(pub usize, pub usize);
fn surface_permutator(mov: Surface) -> Vec<Arrow> {
    vec![
        Arrow(pos(mov, 0, 0), pos(mov, 0, 2)),
        Arrow(pos(mov, 0, 1), pos(mov, 1, 2)),
        Arrow(pos(mov, 0, 2), pos(mov, 2, 2)),
        Arrow(pos(mov, 1, 0), pos(mov, 0, 1)),
        Arrow(pos(mov, 1, 2), pos(mov, 2, 1)),
        Arrow(pos(mov, 2, 0), pos(mov, 0, 0)),
        Arrow(pos(mov, 2, 1), pos(mov, 1, 0)),
        Arrow(pos(mov, 2, 2), pos(mov, 2, 0)),
    ]
}
fn edge_permutator(edges: [(Surface, [(usize, usize); 3]); 4]) -> Vec<Arrow> {
    let mut v = vec![];
    for k in 0..4 {
        let (surface_x, edges_x) = edges[k];
        let (surface_y, edges_y) = edges[(k + 1) % 4];
        for i in 0..3 {
            v.push(Arrow(
                pos(surface_x, edges_x[i].0, edges_x[i].1),
                pos(surface_y, edges_y[i].0, edges_y[i].1),
            ));
        }
    }
    v
}
fn from_arrows(arrows: Vec<Arrow>) -> PermutationMatrix {
    let mut perm = [0; 54];
    for j in 0..54 {
        perm[j] = j;
    }
    for Arrow(from, to) in arrows {
        perm[from] = to;
    }
    PermutationMatrix::op(Permutation::new(perm))
}
fn concat(mut x: Vec<Arrow>, mut y: Vec<Arrow>) -> Vec<Arrow> {
    x.append(&mut y);
    x
}
use std::collections::HashMap;
struct Cache {
    memo: HashMap<(Move, i8), PermutationMatrix>,
}
impl Cache {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }
    fn get(&mut self, mov: Move, rep: i8) -> PermutationMatrix {
        use Surface::*;
        if let Some(v) = self.memo.get(&(mov, rep)) {
            return *v;
        }
        let v = match (mov, rep) {
            (Move::U, 1) => {
                let s = surface_permutator(U);
                let e = edge_permutator([
                    (F, [(0, 0), (0, 1), (0, 2)]),
                    (L, [(0, 0), (0, 1), (0, 2)]),
                    (B, [(0, 2), (1, 2), (2, 2)]),
                    (R, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::D, 1) => {
                let s = surface_permutator(D);
                let e = edge_permutator([
                    (F, [(2, 2), (2, 1), (2, 0)]),
                    (R, [(2, 0), (1, 0), (0, 0)]),
                    (B, [(2, 0), (1, 0), (0, 0)]),
                    (L, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::F, 1) => {
                let s = surface_permutator(F);
                let e = edge_permutator([
                    (U, [(0, 2), (1, 2), (2, 2)]),
                    (R, [(0, 0), (0, 1), (0, 2)]),
                    (D, [(0, 0), (0, 1), (0, 2)]),
                    (L, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::B, 1) => {
                let s = surface_permutator(B);
                let e = edge_permutator([
                    (U, [(2, 0), (1, 0), (0, 0)]),
                    (L, [(2, 0), (1, 0), (0, 0)]),
                    (D, [(2, 2), (2, 1), (2, 0)]),
                    (R, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::R, 1) => {
                let s = surface_permutator(R);
                let e = edge_permutator([
                    (F, [(0, 2), (1, 2), (2, 2)]),
                    (U, [(0, 0), (0, 1), (0, 2)]),
                    (B, [(0, 0), (0, 1), (0, 2)]),
                    (D, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::L, 1) => {
                let s = surface_permutator(L);
                let e = edge_permutator([
                    (U, [(2, 2), (2, 1), (2, 0)]),
                    (F, [(2, 0), (1, 0), (0, 0)]),
                    (D, [(2, 0), (1, 0), (0, 0)]),
                    (B, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            (Move::E, 1) => {
                let e = edge_permutator([
                    (F, [(1, 2), (1, 1), (1, 0)]),
                    (R, [(2, 1), (1, 1), (0, 1)]),
                    (B, [(2, 1), (1, 1), (0, 1)]),
                    (L, [(1, 2), (1, 1), (1, 0)]),
                ]);
                from_arrows(e)
            }
            (Move::S, 1) => {
                let e = edge_permutator([
                    (R, [(1, 0), (1, 1), (1, 2)]),
                    (D, [(1, 0), (1, 1), (1, 2)]),
                    (L, [(0, 1), (1, 1), (2, 1)]),
                    (U, [(0, 1), (1, 1), (2, 1)]),
                ]);
                from_arrows(e)
            }
            (Move::M, 1) => {
                let e = edge_permutator([
                    (U, [(1, 2), (1, 1), (1, 0)]),
                    (F, [(2, 1), (1, 1), (0, 1)]),
                    (D, [(2, 1), (1, 1), (0, 1)]),
                    (B, [(1, 2), (1, 1), (1, 0)]),
                ]);
                from_arrows(e)
            }
            (Move::u, 1) => self.get(Move::U, 1) * self.get(Move::E, -1),
            (Move::d, 1) => self.get(Move::D, 1) * self.get(Move::E, 1),
            (Move::f, 1) => self.get(Move::F, 1) * self.get(Move::S, 1),
            (Move::b, 1) => self.get(Move::B, 1) * self.get(Move::S, -1),
            (Move::r, 1) => self.get(Move::R, 1) * self.get(Move::M, -1),
            (Move::l, 1) => self.get(Move::L, 1) * self.get(Move::M, 1),
            (Move::x, 1) => self.get(Move::r, 1) * self.get(Move::L, -1),
            (Move::y, 1) => self.get(Move::u, 1) * self.get(Move::D, -1),
            (Move::z, 1) => self.get(Move::f, 1) * self.get(Move::B, -1),
            (mov, rep) => {
                let mut mat = PermutationMatrix::identity();
                for _ in 0..(rep + 4) {
                    mat = self.get(mov, 1) * mat;
                }
                mat
            }
        };
        self.memo.insert((mov, rep), v);
        v
    }
}

static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache::new();
    Mutex::new(cache)
});
/// Get permutation from a `Command`.
pub fn of(c: Command) -> PermutationMatrix {
    let mut cache = CACHE.lock().unwrap();
    cache.get(c.0, c.1)
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_cancel() {
        for mov in MOVE {
            let f = of(Command(mov, 1));
            let g = of(Command(mov, -1));
            assert_eq!(g * f, PermutationMatrix::identity());
        }
    }
    #[test]
    fn test_4times() {
        for mov in MOVE {
            let f = of(Command(mov, 4));
            assert_eq!(f, PermutationMatrix::identity());
        }
    }
    #[test]
    fn test_sexy_move_6times() {
        let mut sexy = PermutationMatrix::identity();
        for com in [
            of(Command(Move::R, 1)),
            of(Command(Move::U, 1)),
            of(Command(Move::R, -1)),
            of(Command(Move::U, -1)),
        ] {
            sexy = com * sexy;
        }
        let mut mat = PermutationMatrix::identity();
        for _ in 0..6 {
            mat = sexy * mat;
        }
        assert_eq!(mat, PermutationMatrix::identity());
    }

    fn arb_op() -> impl Strategy<Value = Move> {
        any::<u32>().prop_map(|x| {
            let i = (x % 18) as usize;
            MOVE[i]
        })
    }
    prop_compose! {
        fn arb_rot()(op in arb_op(), rep in 1..=3) -> Command {
            Command(op, rep as i8)
        }
    }
    proptest! {
        #[test]
        fn test_counter_rot(v in prop::collection::vec(arb_rot(), 500..1000)){
            let mut rev = v.clone();
            rev.reverse();

            let mut mat = PermutationMatrix::identity();
            for c in v {
                mat = of(c) * mat;
            }
            for c in rev {
                mat = of(c.prime()) * mat;
            }
            assert_eq!(mat, PermutationMatrix::identity());
        }
    }
}
