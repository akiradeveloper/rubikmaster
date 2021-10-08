use matrix::PermutationMatrix;
use rand::Rng;
use rubikmaster::coord;
use rubikmaster::*;
use std::collections::HashSet;

fn apply_prime(m: PermutationMatrix, seq: &str) -> (PermutationMatrix, Vec<Command>) {
    let mut m = m;
    let pll = parser::parse(&seq).unwrap();
    let pll = flatten(pll.1);
    let pll_prime = flatten(vec![Elem::Group(pll.clone(), -1)]);
    for x in pll_prime {
        let op = matrix::of(x);
        m = op * m;
    }
    (m, pll)
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut init_state = matrix::PermutationMatrix::identity();

    // PLL
    let x: usize = rng.gen();
    let pll = cfop::PLL_LIST[x % 21];
    init_state = apply_prime(init_state, &pll.1).0;

    // OLL
    let y: usize = rng.gen();
    let oll = cfop::OLL_LIST[x % 57];
    init_state = apply_prime(init_state, &oll).0;

    // F2L
    let z: usize = rng.gen();
    let f2l = cfop::F2L_LIST[x % 3];
    let (init_state, solve) = apply_prime(init_state, &f2l);

    let c0 = coord::Surface::D as u8;
    let k1 = coord::surface_number(coord::Surface::R, 1, 1);
    let c1 = init_state.inv_perm[k1 as usize] / 9;
    let k2 = coord::surface_number(coord::Surface::F, 1, 1);
    let c2 = init_state.inv_perm[k2 as usize] / 9;
    let allow_color_list: HashSet<u8> = vec![c0, c1, c2].into_iter().collect();

    let mut blacklist = HashSet::new();
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                let p = coord::Piece(x, y, z);
                let mut surfaces = vec![];

                // list all valid surface numbers
                for sur in coord::SURFACE_LIST {
                    let idx = coord::surface_index_of(p, sur);
                    if let Some(coord::SurfaceIndex(s, i, j)) = idx {
                        let k = coord::surface_number(s, i, j);
                        surfaces.push(k);
                    }
                }
                // check if every surfaces are allowed.
                let mut ok = true;
                for &k in &surfaces {
                    let c = k / 9;
                    if !allow_color_list.contains(&c) {
                        ok = false;
                    }
                }
                if !ok {
                    for k in surfaces {
                        blacklist.insert(k);
                    }
                }
            }
        }
    }
    yew::services::ConsoleService::log(&format!("{:?}", blacklist));

    yew::start_app_with_props::<component::Cube>(component::Props {
        init_state,
        command_list: vec![],
        blacklist,
    });
}
