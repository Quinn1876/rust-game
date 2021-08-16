use gl;
use crate::glm_ext::CachedMatrix4;
use crate::camera::Camera;

pub struct ModelViewProjectionMatrix {
    gl: gl::Gl,
    gl_matrix_id: Option<gl::types::GLint>,

    model: CachedMatrix4,           // Moves the object into the world space
}


impl ModelViewProjectionMatrix {
    /**
     * Registers the matrix with a given opengl program id and links it to a "uniform" (variable passed into shaders)
     * which can be accessed from inside of shader files that are associated with that program.
     */
    pub fn register_with_program_and_uniform(&mut self, program_id: gl::types::GLuint, uniform_name: &[u8])
    {
        if !uniform_name.ends_with(&[b'\0']) {
            panic!("Uniform must be null terminated")
        }

        self.gl_matrix_id = Some(unsafe {
            self.gl.GetUniformLocation(
                program_id,
                uniform_name.as_ptr() as *const gl::types::GLchar
            )
        });
    }


    /**
     *  Calculates and returns the ModelViewProjection matrix
     *
     * Only calculates the matrix if one of either the model, the view, or the projection
     * matrix have changed.
     * If a new matrix is calculated, then the Uniform is updated. OTherwise nothing happends
     */
    pub fn calculate_and_update_mvp(&mut self, camera: &mut Camera)
    {
        if let Some(matrix_id) = self.gl_matrix_id {
            if self.model.is_dirty() || camera.is_projection_view_dirty() {
                let mvp = camera.get_projection_view_matrix() * self.model.matrix;
                self.model.clean();
                camera.clean_projection_view();

                unsafe {
                    self.gl.UniformMatrix4fv(
                        matrix_id,                      // location
                        1,                              // count
                        gl::FALSE,                      // does the matrix need to be transposed?
                        glm::value_ptr(&mvp).as_ptr()   // The matrix which is being passed in
                    );
                }
            }
        } else {
            panic!("Error: Attempting to update the mvp for an unregistered matrix")
        }
    }

    /**
     * @brief: This creates a new ModelViewProjectionMatrix
     * The model view projection contains the information for a model
     * and then combines that information with a camera to provide
     * a matrix which can be used to render the model to the screen
     */
    pub fn new<'camera>(
        gl: &gl::Gl,
    )
    -> ModelViewProjectionMatrix
    {
        ModelViewProjectionMatrix {
            gl: gl.clone(),
            gl_matrix_id: None,
            model: glm::identity::<f32, glm::U4>().into(),
        }
    }
}
