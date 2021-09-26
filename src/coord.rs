//! Functions about Cube's coordinate system.

use crate::{Surface, SURFACE_LIST};

/// Give unique index to a position in a surface.
pub fn surface_number(surface: Surface, i: u8, j: u8) -> u8 {
    let n = surface as u8;
    (9 * n + 3 * i + j) as u8
}
/// Inverse function of `surface_number`.
pub fn surface_number_inv(mut k: u8) -> (Surface, u8, u8) {
    let n = k / 9;
    k -= 9 * n;
    let i = k / 3;
    k -= 3 * i;
    let j = k;
    (SURFACE_LIST[n as usize], i, j)
}
#[test]
fn test_pos() {
    assert_eq!(surface_number(Surface::F, 2, 2), 44);
    assert_eq!(surface_number_inv(44), (Surface::F, 2, 2));
}
