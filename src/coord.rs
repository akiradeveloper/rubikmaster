//! Functions about Cube's coordinate system.

use crate::{Surface, SURFACE_LIST};

/// Give unique index to a position in a surface.
pub fn surface_number(surface: Surface, i: usize, j: usize) -> usize {
    let n = surface as usize;
    9 * n + 3 * i + j
}
/// Inverse function of `surface_number`.
pub fn surface_number_inv(mut k: usize) -> (Surface, usize, usize) {
    let n = k / 9;
    k -= 9 * n;
    let i = k / 3;
    k -= 3 * i;
    let j = k;
    (SURFACE_LIST[n], i, j)
}
#[test]
fn test_pos() {
    assert_eq!(surface_number(Surface::F, 2, 2), 44);
    assert_eq!(surface_number_inv(44), (Surface::F, 2, 2));
}
