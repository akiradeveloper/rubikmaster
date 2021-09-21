//! Parse ratation sequence like (RUR')U'(R'FR)F'

use crate::Command;

pub enum Elem {
    One(Command),
    Group(Vec<Command>, i8),
}
pub fn parse(s: &str) -> Vec<Elem> {
    todo!()
}
