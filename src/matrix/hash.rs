const N: usize = 10;
fn perm_hash(p: [u8;N]) -> u64 {
	let mut ans = 0;
	for i in 0..N {
		let x = (i+1) as u64;
		let y = (p[i]+1) as u64;
		ans *= x+y;
	}
	ans
}
#[test]
fn test_hash_perm() {
	use itertools::Itertools;
	use std::collections::HashSet;
	let mut m = HashSet::new();
	for p in (0..N).permutations(N) {
		let mut q = [0u8;N];
		for i in 0..N {
			q[i] = p[i] as u8;
		}
		let h = perm_hash(q);
		assert!(!m.contains(&h));
		m.insert(h);
	}
}