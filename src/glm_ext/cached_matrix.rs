use nalgebra_glm as glm;

pub struct CachedMatrix4 {
    pub matrix: glm::Mat4,
    pub dirty: bool,
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

    pub fn set_matrix(&mut self, newMatrix: glm::Mat4) {
        self.matrix = newMatrix;
        self.dirty = true;
    }
}
