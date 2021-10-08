//! Yew component to visualize Rubik's cube.

use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use web_sys::WebGl2RenderingContext as GL;
use yew::services::{RenderService, Task};
use yew::{html, Component, ComponentLink, Html, NodeRef, ShouldRender};

use crate::coord::*;
use crate::matrix::*;
use crate::*;
use nalgebra_glm::{vec3, vec4, Vec3, Vec4};
use std::collections::{HashSet, VecDeque};

struct CubePiece {
    pub vertices: [Vec3; 8],
}
impl CubePiece {
    fn new(center: Vec3, edge: f32) -> Self {
        let e = 0.5 * edge;
        let mut vertices = [Vec3::default(); 8];
        for bits in 0..8 {
            let x = if bits & (1 << 2) > 0 { 1. } else { -1. };
            let y = if bits & (1 << 1) > 0 { 1. } else { -1. };
            let z = if bits & (1 << 0) > 0 { 1. } else { -1. };
            let diff = vec3(x * e, y * e, z * e);
            vertices[bits] = center + diff;
        }
        Self { vertices }
    }
}
#[test]
fn test_cube_piece() {
    let c = CubePiece::new(vec3(0., 0., 0.), 2.);
    dbg!(c.vertices);
}

// Counter-Clockwise
const SURFACE_INDICES: [[u8; 4]; 6] = [
    [0b100, 0b110, 0b111, 0b101], // R (x=1)
    [0b000, 0b001, 0b011, 0b010], // L (x=0)
    [0b010, 0b011, 0b111, 0b110], // U (y=1)
    [0b000, 0b100, 0b101, 0b001], // D (y=0)
    [0b001, 0b101, 0b111, 0b011], // F (z=1)
    [0b000, 0b010, 0b110, 0b100], // B (z=0)
];

fn make_color_list(colors: [Vec4; 6]) -> [Vec4; 54] {
    let mut out = [Vec4::default(); 54];
    for surface in SURFACE_LIST {
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let p = Piece(x, y, z);
                    if let Some(SurfaceIndex(s, i, j)) = surface_index_of(p, surface) {
                        let b = surface_number(s, i, j);
                        out[b as usize] = colors[surface as usize];
                    }
                }
            }
        }
    }
    out
}

const TIMEMS_90DEGREE: f64 = 200.;
#[derive(Debug)]
struct RotationProgress {
    pieces: HashSet<Piece>,
    axis: Axis,
    clockwise: i8,
    complete_angle: f64,
    start_time: f64,
}
impl RotationProgress {
    fn cur_angle(&self, timestamp: f64) -> f64 {
        let elapsed = timestamp - self.start_time;
        let direction = if self.clockwise > 0 { 1. } else { -1. };
        direction * 1.57 * elapsed / TIMEMS_90DEGREE
    }
}

pub struct Cube {
    canvas: Option<HtmlCanvasElement>,
    gl: Option<GL>,
    link: ComponentLink<Self>,
    node_ref: NodeRef,
    render_loop: Option<Box<dyn Task>>,

    state: PermutationMatrix,
    color_list: [Vec4; 54],

    command_queue: VecDeque<Command>,
    cur_rotation: Option<RotationProgress>,
    next_state: PermutationMatrix,

    blacklist: HashSet<u8>,
}

pub enum Msg {
    Render(f64),
}

#[derive(yew::Properties, Clone)]
pub struct Props {
    pub init_state: PermutationMatrix,
    #[prop_or_default]
    pub command_list: Vec<Command>,
    #[prop_or_default]
    pub blacklist: HashSet<u8>,
}

impl Component for Cube {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let colors = [
            vec4(0.0, 1.0, 0.5, 1.),
            vec4(0.0, 0.0, 1.0, 1.),
            vec4(1.0, 1.0, 0.0, 1.),
            vec4(1.0, 1.0, 1.0, 1.),
            vec4(1.0, 0.0, 0.0, 1.),
            vec4(1.0, 0.5, 0.0, 1.),
        ];
        let color_list = make_color_list(colors);

        let mut command_queue = VecDeque::new();
        for x in props.command_list {
            command_queue.push_back(x);
        }

        Cube {
            canvas: None,
            gl: None,
            link,
            node_ref: NodeRef::default(),
            render_loop: None,
            state: props.init_state,
            color_list,

            command_queue,
            cur_rotation: None,
            next_state: props.init_state,

            blacklist: props.blacklist,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
            canvas.set_height(300);
            canvas.set_width(300);

            let gl: GL = canvas
                .get_context("webgl2")
                .unwrap()
                .unwrap()
                .dyn_into()
                .unwrap();

            self.canvas = Some(canvas);
            self.gl = Some(gl);

            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);
            self.render_loop = Some(Box::new(handle));
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Render(timestamp) => {
                self.render_gl(timestamp);
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }
}

impl Cube {
    fn render_gl(&mut self, timestamp: f64) {
        let gl = self.gl.as_ref().expect("GL Context not initialized!");

        let should_dequeue = match &self.cur_rotation {
            Some(x) => x.cur_angle(timestamp).abs() >= x.complete_angle.abs(),
            None => true,
        };
        if should_dequeue {
            self.cur_rotation = None;
            self.state = self.next_state;
            if let Some(head) = self.command_queue.pop_front() {
                let rot = coord::rotation_of(head);
                let mut pieces = HashSet::new();
                for i in rot.indices {
                    let plane = coord::RotationPlane(rot.axis, i);
                    for x in coord::piece_group_of(plane) {
                        pieces.insert(*x);
                    }
                }
                let new_rot = RotationProgress {
                    pieces,
                    clockwise: rot.clockwise,
                    axis: rot.axis,
                    complete_angle: 1.57 * rot.clockwise as f64,
                    start_time: timestamp,
                };
                self.cur_rotation = Some(new_rot);
                self.next_state = matrix::of(head) * self.state;
            }
        }

        gl.clear_color(0., 0., 0., 1.);
        gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);
        gl.enable(GL::DEPTH_TEST);

        let vert_code = include_str!("./cube.vert");
        let vert_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vert_shader, &vert_code);
        gl.compile_shader(&vert_shader);

        let frag_code = include_str!("./cube.frag");
        let frag_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&frag_shader, &frag_code);
        gl.compile_shader(&frag_shader);

        let shader_program = gl.create_program().unwrap();
        gl.attach_shader(&shader_program, &vert_shader);
        gl.attach_shader(&shader_program, &frag_shader);
        gl.link_program(&shader_program);

        gl.use_program(Some(&shader_program));

        let canvas = self.canvas.as_ref().unwrap();
        let width = canvas.width();
        let height = canvas.height();
        gl.viewport(0, 0, width as i32, height as i32);

        let eye = vec3(15., 15., 15.);
        let m_model_view = nalgebra_glm::look_at(&eye, &vec3(0., 0., 0.), &vec3(0., 1., 0.));
        let u_model_view_ref = gl.get_uniform_location(&shader_program, "u_model_view");
        gl.uniform_matrix4fv_with_f32_array(
            u_model_view_ref.as_ref(),
            false,
            m_model_view.as_slice(),
        );

        let m_projection = nalgebra_glm::perspective(width as f32 / height as f32, 0.5, 5., 30.);
        let u_projection_ref = gl.get_uniform_location(&shader_program, "u_projection");
        gl.uniform_matrix4fv_with_f32_array(
            u_projection_ref.as_ref(),
            false,
            m_projection.as_slice(),
        );

        let edge = 2.;
        let e = 0.5 * edge;
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    let piece = Piece(x, y, z);
                    let piece_center = 2. * vec3(x as f32 * e, y as f32 * e, z as f32 * e);
                    let cube = CubePiece::new(piece_center, 0.95 * edge);
                    let mut vertex_pos_list = vec![];
                    let mut vertex_color_list = vec![];
                    let mut index_list = vec![];
                    let mut offset: u16 = 0;
                    for surface in SURFACE_LIST {
                        let indices = SURFACE_INDICES[surface as usize];
                        for i in indices {
                            let v = cube.vertices[i as usize];
                            vertex_pos_list.push(v[0]);
                            vertex_pos_list.push(v[1]);
                            vertex_pos_list.push(v[2]);
                        }

                        let sur = surface_index_of(piece, surface);
                        let color = match sur {
                            None => vec4(0., 0., 0., 1.),
                            Some(SurfaceIndex(s, i, j)) => {
                                let k = surface_number(s, i, j);
                                let k = self.state.inv_perm[k as usize];
                                if self.blacklist.contains(&k) {
                                    vec4(0., 0., 0., 0.6)
                                } else {
                                    self.color_list[k as usize]
                                }
                            }
                        };
                        for _ in 0..4 {
                            vertex_color_list.push(color[0]);
                            vertex_color_list.push(color[1]);
                            vertex_color_list.push(color[2]);
                            vertex_color_list.push(color[3]);
                        }

                        for i in [0, 1, 2, 0, 2, 3] {
                            index_list.push(offset + i);
                        }
                        offset += 4;
                    }

                    let identity = nalgebra_glm::Mat4::identity();
                    let m_rotation = if let Some(cur_rot) = &self.cur_rotation {
                        if cur_rot.pieces.contains(&piece) {
                            match cur_rot.axis {
                                Axis::X => nalgebra_glm::rotate_x(
                                    &identity,
                                    // The angle is opposite to cube rotation.
                                    -cur_rot.cur_angle(timestamp) as f32,
                                ),
                                Axis::Y => nalgebra_glm::rotate_y(
                                    &identity,
                                    -cur_rot.cur_angle(timestamp) as f32,
                                ),
                                Axis::Z => nalgebra_glm::rotate_z(
                                    &identity,
                                    -cur_rot.cur_angle(timestamp) as f32,
                                ),
                            }
                        } else {
                            identity
                        }
                    } else {
                        identity
                    };
                    let u_rotation_ref = gl.get_uniform_location(&shader_program, "u_rotation");
                    gl.uniform_matrix4fv_with_f32_array(
                        u_rotation_ref.as_ref(),
                        false,
                        m_rotation.as_slice(),
                    );

                    let vertex_buffer = gl.create_buffer().unwrap();
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertex_buffer));
                    let tmp = js_sys::Float32Array::from(vertex_pos_list.as_slice());
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &tmp, GL::STATIC_DRAW);
                    let v_in_position_ref =
                        gl.get_attrib_location(&shader_program, "v_in_position") as u32;
                    gl.vertex_attrib_pointer_with_i32(v_in_position_ref, 3, GL::FLOAT, false, 0, 0);
                    gl.enable_vertex_attrib_array(v_in_position_ref);
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);

                    let color_buffer = gl.create_buffer().unwrap();
                    gl.bind_buffer(GL::ARRAY_BUFFER, Some(&color_buffer));
                    let tmp = js_sys::Float32Array::from(vertex_color_list.as_slice());
                    gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &tmp, GL::STATIC_DRAW);
                    let v_in_color_ref =
                        gl.get_attrib_location(&shader_program, "v_in_color") as u32;
                    gl.vertex_attrib_pointer_with_i32(v_in_color_ref, 4, GL::FLOAT, false, 0, 0);
                    gl.enable_vertex_attrib_array(v_in_color_ref);
                    gl.bind_buffer(GL::ARRAY_BUFFER, None);

                    let index_buffer = gl.create_buffer().unwrap();
                    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));
                    let tmp = js_sys::Uint16Array::from(index_list.as_slice());
                    gl.buffer_data_with_array_buffer_view(
                        GL::ELEMENT_ARRAY_BUFFER,
                        &tmp,
                        GL::STATIC_DRAW,
                    );
                    gl.draw_elements_with_i32(GL::TRIANGLES, 36, GL::UNSIGNED_SHORT, 0);
                    gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, None);
                }
            }
        }

        if self.cur_rotation.is_some() {
            let render_frame = self.link.callback(Msg::Render);
            let handle = RenderService::request_animation_frame(render_frame);

            // A reference to the new handle must be retained for the next render to run.
            self.render_loop = Some(Box::new(handle));
        }
    }
}
