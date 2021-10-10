use matrix::PermutationMatrix;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::Rng;
use rubikmaster::component::*;
use rubikmaster::coord;
use rubikmaster::*;
use std::collections::HashSet;
use yew::services::ConsoleService;
use yew::*;

struct Problem {
    no: usize,
    state: PermutationMatrix,
    solve: Vec<Command>,
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
        solve,
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
            cur_problem: make_problem(0),
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
