use gl;
use nalgebra_glm as glm;

/**
 * The Camera Struct will manage the view
 * and projection attributes for the ModelViewProjection
 * rendering system described here: http://www.opengl-tutorial.org/beginners-tutorials/tutorial-3-matrices/
 *
 * There may be a need for some default camera constructors in the future,
 * such as common ones used for a menu screen, but that will be a later problem.
 */

 pub struct Camera {
    fov: f32,                   // field of view - the breadth of the angle of things that you can see to the sides of the camera
    aspect_ratio: f32,          // height / width
    near_clipping_plane: f32,   // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html
    far_clipping_plane: f32,    // https://knowledge.autodesk.com/support/maya/learn-explore/caas/CloudHelp/cloudhelp/2018/ENU/Maya-Rendering/files/GUID-D69C23DA-ECFB-4D95-82F5-81118ED41C95-htm.html
    camera_position: glm::Vec3, // position in space of the camera
    center_of_view: glm::Vec3,  // the center point of the camera's view
    up_vector: glm::Vec3        // Generally one of two states, either (0, 1, 0) for normal viewing, or (0,-1,0) for upside down viewing
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
        Camera {
            fov: f32,
            aspect_ratio: f32,
            near_clipping_plane: f32,
            far_clipping_plane: f32,
            camera_position: glm::Vec3,
            center_of_view: glm::Vec3,
            up_vector: glm::Vec3
        }
    }
}
