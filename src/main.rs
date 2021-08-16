// Tutorial from http://nercury.github.io/rust/opengl/tutorial/2018/02/12/opengl-in-rust-from-scratch-06-gl-generator.html
// Reference sites for crates
// nalgebra.org
// ncollide.org
// nphysics.org

#[macro_use] extern crate failure;
#[macro_use] extern crate render_gl_derive;

extern crate sdl2;
extern crate gl;
extern crate vec_2_10_10_10;
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

pub mod render_gl;
pub mod resources;
pub mod mvp_matrix;
pub mod glm_ext;
pub mod camera;

mod triangle;
mod debug;

use std::path::Path;

use resources::Resources;
use render_gl::data;
use debug::failure_to_string;
use camera::Camera;

const WINDOW_HEIGHT: u32 = 700;
const WINDOW_WIDTH: u32 = 900;
const OPENGL_CORE_VERSION_MAJOR: u8 = 4;
const OPENGL_CORE_VERSION_MINOR: u8 = 5;
const WINDOW_TITLE: &str = "Game";

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

fn main() {
    if let Err(e) = run() {
        println!("{}", failure_to_string(e));
    }
}


fn run() -> Result<(), failure::Error> {
    let res = Resources::from_relative_exe_path(Path::new("assets-07")).unwrap();

    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(OPENGL_CORE_VERSION_MAJOR, OPENGL_CORE_VERSION_MINOR);

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl() // adds an opengl flag
        .resizable()
        .build()
        .unwrap();

    let mut viewport = render_gl::Viewport::for_window(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

    let color_buffer = render_gl::ColorBuffer::from_color(na::Vector3::new(0.3, 0.3, 0.5));

    let mut event_pump = sdl.event_pump().unwrap();
    let gl_context = window.gl_create_context().unwrap();
    let gl = gl::Gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    viewport.set_used(&gl);
    color_buffer.set_used(&gl);

    let mut camera = Camera::new(
        45.0 * glm::pi::<f32>() / 180.0,
        viewport.h as f32 / viewport.w as f32,
        0.1,
        100.0,
        glm::vec3(4.0, 3.0, 3.0),
        glm::vec3(0.0, 0.0, 0.0),
        glm::vec3(0.0, 1.0, 0.0)
    );

    let mut triangle = triangle::Triangle::new(res, &gl)?;

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::Window {
                    win_event: sdl2::event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    viewport.update_size(w, h);
                    viewport.set_used(&gl);
                }
                _ => {}
            }

        }

        color_buffer.clear(&gl);

        triangle.render(&gl, &mut camera);

        window.gl_swap_window();
    }
    Ok(())
}
