use matrix::PermutationMatrix;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;
use rubikmaster::component::*;
use rubikmaster::coord;
use rubikmaster::*;
use std::collections::HashMap;
use std::collections::HashSet;
use yew::services::ConsoleService;
use yew::*;

use coord::Axis::*;
const NO_ROT: [((coord::Axis, i8), (coord::Axis, i8)); 3] =
    [((X, 1), (X, 1)), ((Y, 1), (Y, 1)), ((Z, 1), (Z, 1))];
const Y_ROT: [((coord::Axis, i8), (coord::Axis, i8)); 3] =
    [((X, 1), (Z, -1)), ((Y, 1), (Y, 1)), ((Z, 1), (X, 1))];

#[derive(Clone, Debug)]
struct SwitchFaceMatrix {
    map: HashMap<(coord::Axis, i8), (coord::Axis, i8)>,
}
impl SwitchFaceMatrix {
    fn new(tbl: [((coord::Axis, i8), (coord::Axis, i8)); 3]) -> Self {
        let mut x = Self {
            map: HashMap::new(),
        };
        for (a, b) in tbl {
            x.register(a, b);
        }
        x
    }
    fn inv(self) -> Self {
        let mut new_map = HashMap::new();
        for (a, b) in self.map {
            new_map.insert(b, a);
        }
        Self { map: new_map }
    }
    fn register(&mut self, a: (coord::Axis, i8), b: (coord::Axis, i8)) {
        let a_rev = (a.0, -1 * a.1);
        let b_rev = (b.0, -1 * b.1);
        self.map.insert(a, b);
        self.map.insert(a_rev, b_rev);
    }
    fn apply(self, other: Self) -> Self {
        let mut new_map = HashMap::new();
        for (a1, b1) in other.map {
            dbg!(b1);
            let b2 = *self.map.get(&b1).unwrap();
            new_map.insert(a1, b2);
        }
        Self { map: new_map }
    }
    fn interpret(&self, axis: coord::Axis, clockwise: i8, indices: u8) -> (coord::Axis, i8, u8) {
        let sign = if clockwise > 0 { 1 } else { -1 };
        let r = *self.map.get(&(axis, sign)).unwrap();
        let rev = sign * r.1;
        let indices = if rev < 0 { rev3bits(indices) } else { indices };
        (r.0, rev * clockwise, indices)
    }
}
fn rev3bits(x: u8) -> u8 {
    let mut ret = 0;
    for i in 0..3 {
        if x & (1 << i) > 0 {
            ret |= 1 << (2 - i);
        }
    }
    ret
}
#[test]
fn switch_face_mat_mul() {
    let id = SwitchFaceMatrix::new(NO_ROT);
    let m1 = SwitchFaceMatrix::new(Y_ROT);
    let m2 = m1.clone().inv();
    let m3 = m2.clone() * m1.clone();
    let m4 = m1 * id.clone();
    let m5 = m2 * id;
}
#[test]
fn test_rev3bits() {
    assert_eq!(rev3bits(0b100), 0b001);
    assert_eq!(rev3bits(0b110), 0b011);
    assert_eq!(rev3bits(0b101), 0b101);
}
impl std::ops::Mul for SwitchFaceMatrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.apply(rhs)
    }
}
fn y_interpret(seq: Vec<Command>) -> Vec<coord::Rotation> {
    let mut ret = vec![];
    let mut m = SwitchFaceMatrix::new(NO_ROT);
    for x in seq {
        match x {
            Command(Move::y, 1) => {
                let op = SwitchFaceMatrix::new(Y_ROT);
                m = op * m;
            }
            Command(Move::y, -1) => {
                let op = SwitchFaceMatrix::new(Y_ROT).inv();
                m = op * m;
            }
            x => {
                let rot = coord::rotation_of(x);
                let (axis, clockwise, indices) = m.interpret(rot.axis, rot.clockwise, rot.indices);
                let new_rot = coord::Rotation {
                    axis,
                    clockwise,
                    indices,
                };
                ret.push(new_rot);
            }
        }
    }
    ret
}

struct Problem {
    no: usize,
    state: PermutationMatrix,
    solve: Vec<coord::Rotation>,
    solve_seq: String,
    ok: bool,
}

fn apply_prime(m: PermutationMatrix, seq: &str) -> (PermutationMatrix, Vec<Command>) {
    let mut m = m;
    let pll = parser::parse(&seq).unwrap();
    let pll = flatten(pll.1);
    let pll_prime = flatten(vec![Elem::Group(pll.clone(), -1)]);
    for x in pll_prime {
        let op = matrix::of(coord::rotation_of(x));
        m = op * m;
    }
    (m, pll)
}

fn make_problem(i: usize) -> Problem {
    ConsoleService::info(&format!("next=No.{}", i + 1));
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
    let f2l = cfop::F2L_LIST[i];
    let (init_state, solve) = apply_prime(init_state, &f2l);

    Problem {
        no: i,
        state: init_state,
        solve: y_interpret(solve),
        solve_seq: f2l.to_owned(),
        ok: true,
    }
}

fn calc_blacklist(m: &PermutationMatrix) -> HashSet<u8> {
    let c0 = coord::Surface::D as u8;
    let k1 = coord::surface_number(coord::Surface::R, 1, 1);
    let c1 = m.inv_perm[k1 as usize] / 9;
    let k2 = coord::surface_number(coord::Surface::F, 1, 1);
    let c2 = m.inv_perm[k2 as usize] / 9;
    let allow_color_list: HashSet<u8> = vec![c0, c1, c2].into_iter().collect();

    let mut blacklist = HashSet::new();
    for x in 0..3 {
        for y in 0..3 {
            for z in 0..3 {
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
    blacklist
}
enum Msg {
    ShowSolve,
    Next,
}
struct App {
    link: ComponentLink<Self>,
    cur_problem: Problem,
    diff_level: [u64; 41],
}
impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: yew::ComponentLink<Self>) -> Self {
        Self {
            link,
            cur_problem: make_problem(28),
            diff_level: [3; 41],
        }
    }

    fn update(&mut self, msg: Self::Message) -> yew::ShouldRender {
        let prob = &mut self.cur_problem;
        match msg {
            Msg::ShowSolve => {
                if prob.ok {
                    prob.ok = false;
                }
            }
            Msg::Next => {
                let diff = &mut self.diff_level[prob.no];
                if prob.ok {
                    if *diff > 0 {
                        *diff -= 1;
                    }
                } else {
                    *diff += 1;
                };
                let dist = WeightedIndex::new(&self.diff_level).ok();
                let i = match dist {
                    Some(x) => {
                        let mut rng = rand::thread_rng();
                        x.sample(&mut rng)
                    }
                    None => 0,
                };
                *prob = make_problem(i);
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> yew::ShouldRender {
        false
    }

    fn view(&self) -> yew::Html {
        let prob = &self.cur_problem;
        let init_state = prob.state;
        let command_list = if prob.ok { vec![] } else { prob.solve.clone() };
        let blacklist = calc_blacklist(&init_state);
        let solve_seq = if prob.ok { "" } else { &prob.solve_seq };
        html! {
            <div>
                <Cube init_state={init_state} command_list={command_list} blacklist = {blacklist} />
                <div>{ format!("no.{}, current difficulty: {}", prob.no+1, self.diff_level[prob.no]) }</div>
                <button onclick=self.link.callback(|_| Msg::Next)>{ "Next" }</button>
                <br/>
                <button onclick=self.link.callback(|_| Msg::ShowSolve)>{ "Solve" }</button>
                <div>{ solve_seq }</div>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>()
}
