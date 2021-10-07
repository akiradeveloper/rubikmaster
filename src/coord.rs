//! Functions about Cube's coordinate system.

use crate::Command;

/// Surface of a cube or a piece.
#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
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

/// Index of a position in a surface.
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct SurfaceIndex(pub Surface, pub u8, pub u8);

/// Give unique index to a position in a surface.
pub fn surface_number(surface: Surface, i: u8, j: u8) -> u8 {
    let n = surface as u8;
    (9 * n + 3 * i + j) as u8
}
/// Inverse function of `surface_number`.
pub fn surface_number_inv(mut k: u8) -> SurfaceIndex {
    let n = k / 9;
    k -= 9 * n;
    let i = k / 3;
    k -= 3 * i;
    let j = k;
    SurfaceIndex(SURFACE_LIST[n as usize], i, j)
}
#[test]
fn test_pos() {
    assert_eq!(surface_number(Surface::F, 2, 2), 44);
    assert_eq!(surface_number_inv(44), SurfaceIndex(Surface::F, 2, 2));
}

/// The index of the piece in range from (-1,-1,-1) to (1,1,1).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece(pub i8, pub i8, pub i8);

/// Get the surface index of a surface of a piece.
pub fn surface_index_of(piece: Piece, surface: Surface) -> Option<SurfaceIndex> {
    use Surface::*;
    let Piece(x, y, z) = piece;
    let pos = match (x, y, z, surface) {
        // x=-1
        (-1, -1, -1, L) => Some((2, 0)),
        (-1, -1, -1, D) => Some((2, 0)),
        (-1, -1, -1, B) => Some((2, 0)),
        (-1, -1, 0, L) => Some((2, 1)),
        (-1, -1, 0, D) => Some((1, 0)),
        (-1, -1, 1, L) => Some((2, 2)),
        (-1, -1, 1, F) => Some((2, 0)),
        (-1, -1, 1, D) => Some((0, 0)),

        (-1, 0, -1, L) => Some((1, 0)),
        (-1, 0, -1, B) => Some((2, 1)),
        (-1, 0, 0, L) => Some((1, 1)),
        (-1, 0, 1, L) => Some((1, 2)),
        (-1, 0, 1, F) => Some((1, 0)),

        (-1, 1, -1, B) => Some((2, 2)),
        (-1, 1, -1, L) => Some((0, 0)),
        (-1, 1, -1, U) => Some((2, 0)),
        (-1, 1, 0, U) => Some((2, 1)),
        (-1, 1, 0, L) => Some((0, 1)),
        (-1, 1, 1, L) => Some((0, 2)),
        (-1, 1, 1, U) => Some((2, 2)),
        (-1, 1, 1, F) => Some((0, 0)),

        // x=0
        (0, -1, -1, B) => Some((1, 0)),
        (0, -1, -1, D) => Some((2, 1)),
        (0, -1, 0, D) => Some((1, 1)),
        (0, -1, 1, D) => Some((0, 1)),
        (0, -1, 1, F) => Some((2, 1)),

        (0, 0, -1, B) => Some((1, 1)),
        (0, 0, 1, F) => Some((1, 1)),

        (0, 1, -1, B) => Some((1, 2)),
        (0, 1, -1, U) => Some((1, 0)),
        (0, 1, 0, U) => Some((1, 1)),
        (0, 1, 1, U) => Some((1, 2)),
        (0, 1, 1, F) => Some((0, 1)),

        // x=1
        (1, -1, -1, R) => Some((2, 0)),
        (1, -1, -1, B) => Some((0, 0)),
        (1, -1, -1, D) => Some((2, 2)),
        (1, -1, 0, R) => Some((1, 0)),
        (1, -1, 0, D) => Some((1, 2)),
        (1, -1, 1, R) => Some((0, 0)),
        (1, -1, 1, F) => Some((2, 2)),
        (1, -1, 1, D) => Some((0, 2)),

        (1, 0, -1, R) => Some((2, 1)),
        (1, 0, -1, B) => Some((0, 1)),
        (1, 0, 0, R) => Some((1, 1)),
        (1, 0, 1, R) => Some((0, 1)),
        (1, 0, 1, F) => Some((1, 2)),

        (1, 1, -1, R) => Some((2, 2)),
        (1, 1, -1, U) => Some((0, 0)),
        (1, 1, -1, B) => Some((0, 2)),
        (1, 1, 0, R) => Some((1, 2)),
        (1, 1, 0, U) => Some((0, 1)),
        (1, 1, 1, U) => Some((0, 2)),
        (1, 1, 1, R) => Some((0, 2)),
        (1, 1, 1, F) => Some((0, 2)),

        _ => None,
    };
    match pos {
        Some((i, j)) => Some(SurfaceIndex(surface, i, j)),
        None => None,
    }
}
#[test]
fn test_surface_index_of() {
    assert_eq!(
        surface_index_of(Piece(0, 1, -1), Surface::B),
        Some(SurfaceIndex(Surface::B, 1, 2))
    );
    assert_eq!(
        surface_index_of(Piece(0, 1, 1), Surface::F),
        Some(SurfaceIndex(Surface::F, 0, 1))
    );
}
#[test]
fn test_surface_index_of_no_dup() {
    use std::collections::HashSet;
    let mut h = HashSet::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for surface in SURFACE_LIST {
                    let a = surface_index_of(Piece(x, y, z), surface);
                    if let Some(a) = a {
                        if h.contains(&a) {
                            panic!("{:?} is duplicated", a);
                        }
                        h.insert(a);
                    }
                }
            }
        }
    }
}

/// Rotation Axis
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// The index of the rotation plane.
/// (X|Y|Z, -1|0|1)
#[derive(PartialEq, Eq, Hash)]
pub struct RotationPlane(pub Axis, pub i8);

use std::collections::HashMap;

use crate::Move;
fn create_plane_group() -> HashMap<RotationPlane, [Piece; 9]> {
    let mut out = HashMap::new();
    for x in -1..=1 {
        let mut list = [Piece(0, 0, 0); 9];
        let mut i = 0;
        for y in -1..=1 {
            for z in -1..=1 {
                list[i] = Piece(x, y, z);
                i += 1;
            }
        }
        out.insert(RotationPlane(Axis::X, x), list);
    }
    for y in -1..=1 {
        let mut list = [Piece(0, 0, 0); 9];
        let mut i = 0;
        for x in -1..=1 {
            for z in -1..=1 {
                list[i] = Piece(x, y, z);
                i += 1;
            }
        }
        out.insert(RotationPlane(Axis::Y, y), list);
    }
    for z in -1..=1 {
        let mut list = [Piece(0, 0, 0); 9];
        let mut i = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                list[i] = Piece(x, y, z);
                i += 1;
            }
        }
        out.insert(RotationPlane(Axis::Z, z), list);
    }
    out
}
static PIECE_GROUP_TBL: Lazy<HashMap<RotationPlane, [Piece; 9]>> =
    Lazy::new(|| create_plane_group());

/// Nine pieces that is a member of rotation plane.
pub fn piece_group_of(plane: RotationPlane) -> &'static [Piece; 9] {
    PIECE_GROUP_TBL.get(&plane).unwrap()
}

/// Representation of rotation corresponding to a `Command`.
#[derive(Clone)]
pub struct Rotation {
    pub axis: Axis,
    pub indices: Vec<i8>,
    pub clockwise: i8,
}
impl Rotation {
    fn repeat(&mut self, n: i8) {
        self.clockwise *= n;
    }
}
const fn rot(axis: Axis, indices: Vec<i8>, clockwise: i8) -> Rotation {
    Rotation {
        axis,
        indices,
        clockwise,
    }
}
struct Cache {
    memo: HashMap<(Move, i8), Rotation>,
}
impl Cache {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }
    fn get(&mut self, mov: Move, rep: i8) -> Rotation {
        if let Some(v) = self.memo.get(&(mov, rep)) {
            return v.clone();
        }
        let v = match (mov, rep) {
            (Move::R, 1) => rot(Axis::X, vec![1], 1),
            (Move::L, 1) => rot(Axis::X, vec![-1], -1),
            (Move::U, 1) => rot(Axis::Y, vec![1], 1),
            (Move::D, 1) => rot(Axis::Y, vec![-1], -1),
            (Move::F, 1) => rot(Axis::Z, vec![1], 1),
            (Move::B, 1) => rot(Axis::Z, vec![-1], -1),
            (Move::r, 1) => rot(Axis::X, vec![0, 1], 1),
            (Move::l, 1) => rot(Axis::X, vec![-1, 0], -1),
            (Move::u, 1) => rot(Axis::Y, vec![0, 1], 1),
            (Move::d, 1) => rot(Axis::Y, vec![-1, 0], -1),
            (Move::f, 1) => rot(Axis::Z, vec![0, 1], 1),
            (Move::b, 1) => rot(Axis::Z, vec![-1, 0], 1),
            (Move::x, 1) => rot(Axis::X, vec![-1, 0, 1], 1),
            (Move::y, 1) => rot(Axis::Y, vec![-1, 0, 1], 1),
            (Move::z, 1) => rot(Axis::Z, vec![-1, 0, 1], 1),
            (Move::M, 1) => rot(Axis::X, vec![0], -1),
            (Move::E, 1) => rot(Axis::Y, vec![0], -1),
            (Move::S, 1) => rot(Axis::Z, vec![0], 1),
            (mov, rep) => {
                let mut rot = self.get(mov, 1);
                rot.repeat(rep);
                rot
            }
        };
        self.memo.insert((mov, rep), v.clone());
        v
    }
}
use once_cell::sync::Lazy;
use std::sync::Mutex;
static CACHE: Lazy<Mutex<Cache>> = Lazy::new(|| {
    let cache = Cache::new();
    Mutex::new(cache)
});
/// Get rotation from a `Command`.
pub fn rotation_of(c: Command) -> Rotation {
    let mut cache = CACHE.lock().unwrap();
    cache.get(c.0, c.1)
}
