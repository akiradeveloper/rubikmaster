/// Permutation i -> p[i]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Permutation {
    inner: [usize; 54],
}
impl Permutation {
    pub fn new(perms: [usize; 54]) -> Self {
        Self { inner: perms }
    }
}
impl std::ops::Index<usize> for Permutation {
    type Output = usize;
    fn index(&self, i: usize) -> &usize {
        &self.inner[i]
    }
}
/// Matrix representation of a `Permutation`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PermutationMatrix {
    pub perms: Permutation,
}
impl PermutationMatrix {
    pub fn identity() -> Self {
        let mut perms = [0; 54];
        for i in 0..54 {
            perms[i] = i;
        }
        PermutationMatrix {
            perms: Permutation::new(perms),
        }
    }
    pub fn new(perms: Permutation) -> Self {
        PermutationMatrix { perms }
    }
    pub fn inv(self) -> Self {
        let mut inv = [0; 54];
        for i in 0..54 {
            inv[self.perms[i]] = i;
        }
        PermutationMatrix {
            perms: Permutation::new(inv),
        }
    }
    fn apply(self, to: Self) -> Self {
        // I think these arguments should be in reverse order but
        // the comparing with reference impl says it's okay.
        let perms = gather(&self.perms.inner, &to.perms.inner);
        Self::new(Permutation::new(perms))
    }
}
impl std::ops::Mul for PermutationMatrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.apply(rhs)
    }
}
fn gather(index: &[usize; 54], v: &[usize; 54]) -> [usize; 54] {
    let mut out = [0; 54];
    for i in 0..54 {
        let k = index[i];
        let j = v[k];
        out[i] = j;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::*;
    use proptest::prelude::*;

    fn arb_perm() -> impl Strategy<Value = Vec<usize>> {
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
    fn to_mat(v: Vec<usize>) -> PermutationMatrix {
        let mut perm = [0; 54];
        for i in 0..54 {
            perm[i] = v[i];
        }
        PermutationMatrix::new(Permutation::new(perm))
    }
    fn to_na_mat(v: Vec<usize>) -> SMatrix<f64, 54, 54> {
        let mut mat: SMatrix<f64, 54, 54> = nalgebra::SMatrix::zeros();
        for i in 0..54 {
            mat[(i, v[i])] = 1.;
        }
        mat
    }
    fn into_mat(m: SMatrix<f64, 54, 54>) -> PermutationMatrix {
        let mut perms = [0; 54];
        for i in 0..54 {
            for j in 0..54 {
                if m[(i, j)] == 1. {
                    perms[i] = j;
                }
            }
        }
        PermutationMatrix::new(Permutation::new(perms))
    }
}
