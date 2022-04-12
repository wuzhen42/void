use crate::prim::{Mat4, Pnt3, Vec3};

#[derive(Default)]
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
    #[rustfmt::skip]
    pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    );

    pub fn build_view_projection_matrix(&self) -> Mat4 {
        let view = Mat4::lookat(self.eye, self.target, self.up);
        let proj = Mat4::perspective(self.fovy, self.aspect, self.znear, self.zfar);
        return Self::OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_proj: [f32; 16],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::I.into(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath;

    #[test]
    fn test_lookat() {
        assert_eq!(
            Mat4::lookat(Pnt3::new(0.0, 1.0, 2.0), Pnt3::ZERO, Vec3::Y),
            Mat4::new(
                1.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.8944271909999159,
                0.4472135954999579,
                0.0,
                0.0,
                -0.4472135954999579,
                0.8944271909999159,
                0.0,
                0.0,
                0.0,
                -2.23606797749979,
                1.0
            )
        );
    }

    #[test]
    fn test_perspective() {
        let a = Mat4::perspective(45.0f64.to_radians(), 4.0 / 3.0, 0.1, 100.0);
        let b = cgmath::perspective(cgmath::Deg(45.0), 4.0 / 3.0, 0.1, 100.0);
        for col in 0..4 {
            for row in 0..4 {
                assert_eq!(a.e(col, row), b[col][row]);
            }
        }
    }
}
