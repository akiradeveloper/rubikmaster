use rubikmaster::*;

fn main() {
    let mut command_list = vec![];
    for c in crate::random(1000) {
        command_list.push(c);
    }
    yew::start_app_with_props::<component::Cube>(component::Props {
        init_state: matrix::PermutationMatrix::identity(),
        command_list,
    });
}
