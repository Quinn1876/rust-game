use nalgebra_glm as glm;
use crate::glm_ext::CachedMatrix4;

/**
 * The Camera Struct will manage the view
 * and projection attributes for the ModelViewProjection
 * rendering system described here: http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/
 *
 * There may be a need for some default camera constructors in the future,
 * such as common ones used for a menu screen, but that will be a later problem.
 */

pub struct Camera {
    // Projection parameters
    fov: f32,                           // field of view - the breadth of the angle of things that you can see to the sides of the camera
    aspect_ratio: f32,                  // height / width
    near_clipping_plane: f32,           // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html
    far_clipping_plane: f32,            // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html

    // View parameters
    camera_position: glm::Vec3,         // position in space of the camera
    center_of_view: glm::Vec3,          // the center point of the camera's view
    up_vector: glm::Vec3,               // Generally one of two states, either (0, 1, 0) for normal viewing, or (0,-1,0) for upside down viewing

    // Matrixes
    projection: CachedMatrix4,
    view: CachedMatrix4,

    projection_view: CachedMatrix4,     // Cached product of projection * view (order does matter)
}

impl Camera {
    pub fn new(
        fov: f32,
        aspect_ratio: f32,
        near_clipping_plane: f32,
        far_clipping_plane: f32,
        camera_position: glm::Vec3,
        center_of_view: glm::Vec3,
        up_vector: glm::Vec3
    )
    -> Camera
    {
        let projection = glm::perspective(aspect_ratio, fov, near_clipping_plane, far_clipping_plane);
        let view = glm::look_at(&camera_position, &center_of_view, &up_vector);
        Camera {
            fov,
            aspect_ratio,
            near_clipping_plane,
            far_clipping_plane,
            camera_position,
            center_of_view,
            up_vector,
            projection: projection.into(),
            view: view.into(),
            projection_view: (projection * view).into()
        }
    }

    pub fn get_projection_view_matrix(&mut self)
    -> glm::Mat4
    {
        if self.projection.is_dirty() || self.view.is_dirty() {
            self.projection_view.set_matrix(self.projection.matrix * self.view.matrix);
            self.view.clean();
            self.projection.clean();
        }

        return self.projection_view.matrix;
    }

    pub fn is_projection_view_dirty(&self)
    -> bool
    {
        self.projection_view.is_dirty()
    }

    pub fn clean_projection_view(&mut self)
    {
        self.projection_view.clean();
    }
}
