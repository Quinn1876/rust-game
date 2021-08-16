use gl;
use nalgebra_glm as glm;

struct CachedMatrix4 {
    matrix: glm::Mat4,
    dirty: bool,
}

impl CachedMatrix4 {
    pub fn is_dirty(&self)
    -> bool
    {
        self.dirty
    }

    pub fn clean(&mut self)
    {
        self.dirty = false;
    }
}

impl From::<glm::Mat4> for CachedMatrix4 {
    fn from(other: glm::Mat4)
    -> CachedMatrix4
    {
        CachedMatrix4 {
            matrix: other,
            dirty: true
        }
    }
}

pub struct ModelViewProjectionMatrix {
    gl: gl::Gl,
    gl_matrix_id: Option<gl::types::GLint>,

    model: CachedMatrix4,           // Moves the object into the world space
    view: CachedMatrix4,            // Changes the origin to the camera
    projection: CachedMatrix4       // changes the transforms other items so that their 3d shape appears properly in 2d space
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
    pub fn calculate_and_update_mvp(&mut self)
    {
        if let Some(matrix_id) = self.gl_matrix_id {
            if self.model.is_dirty() || self.projection.is_dirty() || self.view.is_dirty() {
                let mvp = self.projection.matrix * self.view.matrix * self.model.matrix;
                self.model.clean();
                self.projection.clean();
                self.view.clean();

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
     * Currently, all fields are to be filled individually.
     *
     * This may need to change since there is generally one
     * camera per scene and it would not make sense to have
     * many copies of the view matrix spread around.
     *
     * Similarly, the perspective matrix will likely be something
     * that is set in a settings page or only made once and then
     * forgotten about. This could lead to some optimizations where
     * the view and projection matrix's are combined and then their
     * product is precomputed for all models.
     */
    pub fn new(
        gl: &gl::Gl,
        fov: f32,                   // field of view - the breadth of the angle of things that you can see to the sides of the camera
        aspect_ratio: f32,          // height / width
        near_clipping_plane: f32,   // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html
        far_clipping_plane: f32,    // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html
        camera_position: glm::Vec3, // position in space of the camera
        center_of_view: glm::Vec3,  // the center point of the camera's view
        up_vector: glm::Vec3        // Generally one of two states, either (0, 1, 0) for normal viewing, or (0,-1,0) for upside down viewing
    )
    -> ModelViewProjectionMatrix
    {
        ModelViewProjectionMatrix {
            gl: gl.clone(),
            gl_matrix_id: None,
            projection: glm::perspective(aspect_ratio, fov, near_clipping_plane, far_clipping_plane).into(),
            view: glm::look_at(&camera_position, &center_of_view, &up_vector).into(),
            model: glm::identity::<f32, glm::U4>().into()
        }
    }
}
