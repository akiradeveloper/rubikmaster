pub mod matrix;
mod parser;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Move {
    U,
    D,
    F,
    B,
    R,
    L,
    u,
    d,
    f,
    b,
    r,
    l,
    M,
    E,
    S,
    x,
    y,
    z,
}
pub const MOVE: [Move; 18] = [
    Move::U,
    Move::D,
    Move::F,
    Move::B,
    Move::R,
    Move::L,
    Move::u,
    Move::d,
    Move::f,
    Move::b,
    Move::r,
    Move::l,
    Move::M,
    Move::E,
    Move::S,
    Move::x,
    Move::y,
    Move::z,
];
#[derive(Clone, Copy, Debug)]
pub struct Command(pub Move, pub i8);
impl Command {
    pub fn prime(self) -> Self {
        Command(self.0, -self.1)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Surface {
    U,
    D,
    F,
    B,
    R,
    L,
}
pub const SURFACE: [Surface; 6] = [
    Surface::U,
    Surface::D,
    Surface::F,
    Surface::B,
    Surface::R,
    Surface::L,
];

pub fn random(n: usize) -> Vec<Command> {
    use rand::prelude::*;
    let mut rng = rand::thread_rng();
    let mut v = vec![];
    for _ in 0..n {
        let mov: usize = rng.gen();
        let mov = MOVE[mov % 18];
        let rep: usize = rng.gen();
        let rep = rep % 3 + 1;
        v.push(Command(mov, rep as i8));
    }
    v
}
