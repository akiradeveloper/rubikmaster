//! Cube's state is expressed as permutation matrix
//! and operations are matrix multiplications.

use crate::coord::{self, surface_number, surface_number_inv};
use crate::coord::{Surface, SurfaceIndex, SURFACE_LIST};
use crate::Command;
use crate::{Move, MOVE_LIST};
use once_cell::sync::Lazy;
use std::sync::Mutex;

mod math;
pub use math::{Permutation, PermutationMatrix};

/// Check if the colors on the given `positions` are the same.
pub(crate) fn same_color_check<const N: usize>(
    mat: &PermutationMatrix,
    positions: [u8; N],
) -> bool {
    let inv = &mat.inv_perm;
    let mut color_list = [Surface::B; N];
    for i in 0..N {
        let pos = inv[positions[i] as usize];
        let SurfaceIndex(c, _, _) = surface_number_inv(pos);
        color_list[i] = c;
    }
    let mut b = true;
    for i in 0..N {
        b &= color_list[i] == color_list[(i + 1) % N];
    }
    b
}
fn matof(c: Command) -> PermutationMatrix {
    let rot = crate::coord::rotation_of(c);
    of(rot)
}
#[test]
fn test_same_color_check() {
    use Surface::*;
    let mut m = PermutationMatrix::identity();
    let c = matof(Command(Move::R, 1));
    m = c * m;

    for _ in 0..1000 {
        let l = [
            (U, 0, 0),
            (U, 0, 1),
            (U, 0, 2),
            (F, 0, 0),
            (F, 0, 1),
            (F, 1, 0),
            (F, 1, 1),
            (F, 2, 0),
            (F, 2, 1),
        ];
        let mut list = [0; 9];
        for k in 0..9 {
            let (sur, i, j) = l[k];
            list[k] = coord::surface_number(sur, i, j);
        }
        assert!(same_color_check(&m, list));
        m = matof(Command(Move::x, 1)) * m;
    }
}
struct Arrow(pub u8, pub u8);
fn surface_permutator(mov: Surface) -> Vec<Arrow> {
    vec![
        Arrow(surface_number(mov, 0, 0), surface_number(mov, 0, 2)),
        Arrow(surface_number(mov, 0, 1), surface_number(mov, 1, 2)),
        Arrow(surface_number(mov, 0, 2), surface_number(mov, 2, 2)),
        Arrow(surface_number(mov, 1, 0), surface_number(mov, 0, 1)),
        Arrow(surface_number(mov, 1, 2), surface_number(mov, 2, 1)),
        Arrow(surface_number(mov, 2, 0), surface_number(mov, 0, 0)),
        Arrow(surface_number(mov, 2, 1), surface_number(mov, 1, 0)),
        Arrow(surface_number(mov, 2, 2), surface_number(mov, 2, 0)),
    ]
}
fn edge_permutator(edges: [(Surface, [(u8, u8); 3]); 4]) -> Vec<Arrow> {
    let mut v = vec![];
    for k in 0..4 {
        let (surface_x, edges_x) = edges[k];
        let (surface_y, edges_y) = edges[(k + 1) % 4];
        for i in 0..3 {
            v.push(Arrow(
                surface_number(surface_x, edges_x[i].0, edges_x[i].1),
                surface_number(surface_y, edges_y[i].0, edges_y[i].1),
            ));
        }
    }
    v
}
fn from_arrows(arrows: Vec<Arrow>) -> PermutationMatrix {
    let mut perm = [0u8; 54];
    for j in 0..54 {
        perm[j] = j as u8;
    }
    for Arrow(from, to) in arrows {
        perm[from as usize] = to;
    }
    PermutationMatrix::op(Permutation::new(perm))
}
fn concat(mut x: Vec<Arrow>, mut y: Vec<Arrow>) -> Vec<Arrow> {
    x.append(&mut y);
    x
}
use std::collections::HashMap;
struct Cache {
    memo: HashMap<coord::Rotation, PermutationMatrix>,
}
impl Cache {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }
    fn get(&mut self, rot: coord::Rotation) -> PermutationMatrix {
        use coord::{Axis::*, Rotation};
        use Surface::*;
        if let Some(v) = self.memo.get(&rot) {
            return *v;
        }
        let v = match rot {
            // R
            Rotation {
                axis: X,
                indices: 0b100,
                clockwise: 1,
            } => {
                let s = surface_permutator(R);
                let e = edge_permutator([
                    (F, [(0, 2), (1, 2), (2, 2)]),
                    (U, [(0, 0), (0, 1), (0, 2)]),
                    (B, [(0, 0), (0, 1), (0, 2)]),
                    (D, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            // L
            Rotation {
                axis: X,
                indices: 0b001,
                clockwise: -1,
            } => {
                let s = surface_permutator(L);
                let e = edge_permutator([
                    (U, [(2, 2), (2, 1), (2, 0)]),
                    (F, [(2, 0), (1, 0), (0, 0)]),
                    (D, [(2, 0), (1, 0), (0, 0)]),
                    (B, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            // U
            Rotation {
                axis: Y,
                indices: 0b100,
                clockwise: 1,
            } => {
                let s = surface_permutator(U);
                let e = edge_permutator([
                    (F, [(0, 0), (0, 1), (0, 2)]),
                    (L, [(0, 0), (0, 1), (0, 2)]),
                    (B, [(0, 2), (1, 2), (2, 2)]),
                    (R, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            // D
            Rotation {
                axis: Y,
                indices: 0b001,
                clockwise: -1,
            } => {
                let s = surface_permutator(D);
                let e = edge_permutator([
                    (F, [(2, 2), (2, 1), (2, 0)]),
                    (R, [(2, 0), (1, 0), (0, 0)]),
                    (B, [(2, 0), (1, 0), (0, 0)]),
                    (L, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            // F
            Rotation {
                axis: Z,
                indices: 0b100,
                clockwise: 1,
            } => {
                let s = surface_permutator(F);
                let e = edge_permutator([
                    (U, [(0, 2), (1, 2), (2, 2)]),
                    (R, [(0, 0), (0, 1), (0, 2)]),
                    (D, [(0, 0), (0, 1), (0, 2)]),
                    (L, [(0, 2), (1, 2), (2, 2)]),
                ]);
                from_arrows(concat(s, e))
            }
            // B
            Rotation {
                axis: Z,
                indices: 0b001,
                clockwise: -1,
            } => {
                let s = surface_permutator(B);
                let e = edge_permutator([
                    (U, [(2, 0), (1, 0), (0, 0)]),
                    (L, [(2, 0), (1, 0), (0, 0)]),
                    (D, [(2, 2), (2, 1), (2, 0)]),
                    (R, [(2, 2), (2, 1), (2, 0)]),
                ]);
                from_arrows(concat(s, e))
            }
            // M
            Rotation {
                axis: X,
                indices: 0b010,
                clockwise: -1,
            } => {
                let e = edge_permutator([
                    (U, [(1, 2), (1, 1), (1, 0)]),
                    (F, [(2, 1), (1, 1), (0, 1)]),
                    (D, [(2, 1), (1, 1), (0, 1)]),
                    (B, [(1, 2), (1, 1), (1, 0)]),
                ]);
                from_arrows(e)
            }
            // E
            Rotation {
                axis: Y,
                indices: 0b010,
                clockwise: -1,
            } => {
                let e = edge_permutator([
                    (F, [(1, 2), (1, 1), (1, 0)]),
                    (R, [(2, 1), (1, 1), (0, 1)]),
                    (B, [(2, 1), (1, 1), (0, 1)]),
                    (L, [(1, 2), (1, 1), (1, 0)]),
                ]);
                from_arrows(e)
            }
            // S
            Rotation {
                axis: Z,
                indices: 0b010,
                clockwise: 1,
            } => {
                let e = edge_permutator([
                    (R, [(1, 0), (1, 1), (1, 2)]),
                    (D, [(1, 0), (1, 1), (1, 2)]),
                    (L, [(0, 1), (1, 1), (2, 1)]),
                    (U, [(0, 1), (1, 1), (2, 1)]),
                ]);
                from_arrows(e)
            }
            Rotation {
                axis,
                indices,
                clockwise: 1,
            } if indices.count_ones() == 1 => self
                .get(Rotation {
                    axis,
                    indices,
                    clockwise: -1,
                })
                .inv(),
            Rotation {
                axis,
                indices,
                clockwise: -1,
            } if indices.count_ones() == 1 => self
                .get(Rotation {
                    axis,
                    indices,
                    clockwise: 1,
                })
                .inv(),
            Rotation {
                axis,
                indices,
                clockwise,
            } => {
                let rep = (clockwise + 4) % 4;
                let mut m = PermutationMatrix::identity();
                for i in 0..3 {
                    let j = indices & (1 << i);
                    if j > 0 {
                        let rot = Rotation {
                            axis,
                            indices: j,
                            clockwise: 1,
                        };
                        let op = self.get(rot);
                        for _ in 0..rep {
                            m = op * m;
                        }
                    }
                }
                m
            }
        };
        self.memo.insert(rot, v);
        v
    }
}

static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache::new();
    Mutex::new(cache)
});
/// Get permutation from a `Rotation`.
pub fn of(rot: coord::Rotation) -> PermutationMatrix {
    let mut cache = CACHE.lock().unwrap();
    cache.get(rot)
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_cancel() {
        for mov in MOVE_LIST {
            let f = matof(Command(mov, 1));
            let g = matof(Command(mov, -1));
            assert_ne!(f, g);
            assert_eq!(g * f, PermutationMatrix::identity());
        }
    }
    #[test]
    fn test_4times() {
        for mov in MOVE_LIST {
            let f = matof(Command(mov, 4));
            assert_eq!(f, PermutationMatrix::identity());
        }
    }
    #[test]
    fn test_sexy_move_6times() {
        let mut sexy = PermutationMatrix::identity();
        for com in [
            matof(Command(Move::R, 1)),
            matof(Command(Move::U, 1)),
            matof(Command(Move::R, -1)),
            matof(Command(Move::U, -1)),
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
            MOVE_LIST[i]
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
                mat = matof(c) * mat;
            }
            for c in rev {
                mat = matof(c.prime()) * mat;
            }
            assert_eq!(mat, PermutationMatrix::identity());
        }
    }
}
