use rubikmaster::*;
use std::collections::HashSet;

fn main() {
    let mut command_list = vec![];
    for c in crate::random(1_000_000) {
        command_list.push(coord::rotation_of(c));
    }
    yew::start_app_with_props::<component::Cube>(component::Props {
        init_state: matrix::PermutationMatrix::identity(),
        command_list,
        blacklist: HashSet::new(),
    });
}
