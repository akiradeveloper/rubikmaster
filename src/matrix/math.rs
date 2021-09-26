/// Permutation i -> p[i]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Permutation {
    inner: [u8; 54],
}
impl Permutation {
    pub fn new(perm: [u8; 54]) -> Self {
        Self { inner: perm }
    }
    fn inv(self) -> Self {
        let mut inv = [0; 54];
        for i in 0..54 {
            inv[self.inner[i as usize] as usize] = i;
        }
        Self { inner: inv }
    }
    fn identity() -> Self {
        let mut inner = [0u8; 54];
        for i in 0..54 {
            inner[i] = i as u8;
        }
        Self { inner }
    }
}
impl std::ops::Index<u8> for Permutation {
    type Output = u8;
    fn index(&self, i: u8) -> &u8 {
        &self.inner[i as usize]
    }
}
/// Matrix representation of a `Permutation`.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct PermutationMatrix {
    /// Inversion of permutation.
    pub inv_perm: [u8; 54],
}
impl PermutationMatrix {
    pub fn identity() -> Self {
        PermutationMatrix {
            inv_perm: Permutation::identity().inner,
        }
    }
    pub fn op(perm: Permutation) -> Self {
        PermutationMatrix {
            inv_perm: perm.inv().inner,
        }
    }
    pub fn inv(self) -> Self {
        PermutationMatrix::op(Permutation::new(self.inv_perm))
    }
    fn apply(self, to: Self) -> Self {
        let out = gather(&self.inv_perm, &to.inv_perm);
        Self { inv_perm: out }
    }
}
impl std::ops::Mul for PermutationMatrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.apply(rhs)
    }
}
fn gather(index: &[u8; 54], v: &[u8; 54]) -> [u8; 54] {
    let mut out = [0; 54];
    for i in 0..54 {
        let k = index[i];
        let j = v[k as usize];
        out[i] = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::*;
    use proptest::prelude::*;

    fn arb_perm() -> impl Strategy<Value = Vec<u8>> {
        let mut v = vec![];
        for i in 0..54 {
            v.push(i);
        }
        Just(v).prop_shuffle()
    }
    proptest! {
        #[test]
        fn test_inv(v in arb_perm()) {
            let mat = to_mat(v);
            let inv = mat.inv();
            assert_eq!(inv * mat, PermutationMatrix::identity());
        }

        #[test]
        fn test_mul_ref(x in arb_perm(), y in arb_perm()) {
            let x0 = to_mat(x.clone());
            let y0 = to_mat(y.clone());
            let z0 = x0 * y0;

            let x1 = to_na_mat(x);
            let y1 = to_na_mat(y);
            let z1 = x1 * y1;
            let z1 = into_mat(z1);

            assert_eq!(z0, z1);
        }

        #[test]
        fn test_inv_ref(v in arb_perm()) {
            let x = to_mat(v.clone());
            let x_inv = x.inv();
            let y = to_na_mat(v);
            let y_inv = y.try_inverse().unwrap();
            let y_inv = into_mat(y_inv);

            assert_eq!(x_inv, y_inv);
        }
    }
    fn to_mat(v: Vec<u8>) -> PermutationMatrix {
        let mut perm = [0u8; 54];
        for i in 0..54 {
            perm[i] = v[i];
        }
        PermutationMatrix::op(Permutation::new(perm))
    }
    fn to_na_mat(v: Vec<u8>) -> SMatrix<f64, 54, 54> {
        let mut mat: SMatrix<f64, 54, 54> = nalgebra::SMatrix::zeros();
        for i in 0..54 {
            mat[(v[i] as usize, i)] = 1.;
        }
        mat
    }
    fn into_mat(m: SMatrix<f64, 54, 54>) -> PermutationMatrix {
        let mut perm = [0u8; 54];
        for j in 0..54 {
            for i in 0..54 {
                if m[(i, j)] == 1. {
                    perm[j] = i as u8;
                }
            }
        }
        PermutationMatrix::op(Permutation::new(perm))
    }
}
