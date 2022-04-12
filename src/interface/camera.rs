use crate::prim::{Mat4, Pnt3, Vec3};

pub struct Camera {
    pub eye: Pnt3,
    pub target: Pnt3,
    pub up: Vec3,
    pub aspect: f64,
    pub fovy: f64,
    pub znear: f64,
    pub zfar: f64,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::lookat(self.eye, self.target, self.up);
        let proj = Mat4::perspective(self.fovy, self.aspect, self.znear, self.zfar);
        return proj * view;
    }
}
