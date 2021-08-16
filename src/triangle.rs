use gl;
use failure;

use crate::render_gl::{ self, data, buffer, Viewport };
use crate::resources::Resources;
use crate::mvp_matrix::ModelViewProjectionMatrix;

#[derive(VertexAttribPointers)]
#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = "0"]
    pos: data::f32_f32_f32,
    #[location = "1"]
    clr: data::u2_u10_u10_u10_rev_float,
}

pub struct Triangle {
    program: render_gl::Program,
    _vbo: buffer::ArrayBuffer,
    vao: buffer::VertexArray,
    mvp_matrix: ModelViewProjectionMatrix
}

impl Triangle {
    pub fn new(res: Resources, gl: &gl::Gl, viewport: &Viewport)
    -> Result<Triangle, failure::Error>
    {

        // set up shader program

        let program = render_gl::Program::from_res(gl, res, "shaders/triangle")?;

        let vertices: Vec<Vertex> = vec![
            Vertex{ pos: (0.5, -0.5, 0.0).into(), clr: (1.0, 0.0, 0.0, 1.0).into() }, // bottom right
            Vertex{ pos: (-0.5, -0.5, 0.0).into(), clr: (0.0, 1.0, 0.0, 1.0).into() }, // bottom left
            Vertex{ pos: (0.0, 0.5, 0.0).into(), clr: (0.0, 0.0, 1.0, 1.0).into() } // top
        ];

        let vbo = render_gl::buffer::ArrayBuffer::new(&gl);
        vbo.bind();
        vbo.static_draw_data(&vertices);
        vbo.unbind();


        let vao = render_gl::buffer::VertexArray::new(&gl);

        vao.bind();
        vbo.bind();

        Vertex::vertex_attrib_pointers(&gl);

        vbo.unbind();
        vao.unbind();

        // let matrix_id = unsafe {
        //     gl.GetUniformLocation(
        //         program.id(),
        //         "MVP\0".as_ptr() as *const gl::types::GLchar // Rust strings are not null terminated by default
        //     )
        // };
        // println!("Matrix id: {}", matrix_id);

        let mut mvp_matrix = ModelViewProjectionMatrix::new(
            &gl,
            45.0 * glm::pi::<f32>() / 180.0,
            viewport.h as f32 / viewport.w as f32,
            0.1,
            100.0,
            glm::vec3(4.0, 3.0, 3.0),
            glm::vec3(0.0, 0.0, 0.0),
            glm::vec3(0.0, 1.0, 0.0)
        );
        mvp_matrix.register_with_program_and_uniform(program.id(), b"MVP\0");

        Ok(Triangle {
            program,
            _vbo: vbo,
            vao,
            mvp_matrix,
        })
    }

    pub fn render(&mut self, gl: &gl::Gl)
    {
        // let fov = 45.0 * glm::pi::<f32>() / 180.0;
        // let aspect_ratio = viewport.h as f32/viewport.w as f32;
        // let near = 0.1;
        // let far = 100.0;
        // let perspective_matrix = glm::perspective(aspect_ratio, fov, near, far);

        // let camera_pos = glm::vec3(4.0,3.0,3.0);
        // let center_of_view = glm::vec3(0.0, 0.0, 0.0);
        // let up_vector = glm::vec3(0.0, 1.0, 0.0); // 0, -1, 0 to be upside down
        // let view_matrix: glm::Mat4 = glm::look_at(&camera_pos, &center_of_view, &up_vector);

        // let model_matrix = glm::identity::<f32, glm::U4>(); // the model will be at the origin for now; later replace this with the model's affine transform

        // let model_view_projection_matrix = perspective_matrix * view_matrix * model_matrix;

        // unsafe {
        //     gl.UniformMatrix4fv(self.mvp_matrix_id, 1, gl::FALSE, glm::value_ptr(&model_view_projection_matrix).as_ptr());
        // }

        self.program.set_used();
        self.vao.bind();

        self.mvp_matrix.calculate_and_update_mvp();


        unsafe {
            gl.DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                3 // number of indices to be rendered
            );
        }
    }
}
