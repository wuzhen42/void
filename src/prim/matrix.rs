use super::{Pnt3, Vec3, Vec4};

pub struct Mat4 {
    _e: [f64; 16],
}

impl Mat4 {
    #[rustfmt::skip]
    pub const fn new(
        c0r0: f64, c0r1: f64, c0r2: f64, c0r3: f64,
        c1r0: f64, c1r1: f64, c1r2: f64, c1r3: f64,
        c2r0: f64, c2r1: f64, c2r2: f64, c2r3: f64,
        c3r0: f64, c3r1: f64, c3r2: f64, c3r3: f64
        ) -> Mat4 {
        Mat4 { _e: [
            c0r0, c0r1, c0r2, c0r3,
            c1r0, c1r1, c1r2, c1r3,
            c2r0, c2r1, c2r2, c2r3,
            c3r0, c3r1, c3r2, c3r3,
        ] }
    }

    #[rustfmt::skip]
    pub fn from_columns(c0: Vec4, c1: Vec4, c2: Vec4, c3: Vec4) -> Mat4 {
        Mat4::new(
            c0[0], c1[0], c2[0], c3[0],
            c0[1], c1[1], c2[1], c3[1],
            c0[2], c1[2], c2[2], c3[2],
            c0[3], c1[3], c2[3], c3[3],
        )
    }

    #[rustfmt::skip]
    pub const I: Mat4 = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    );

    pub fn col(&self, i: usize) -> Vec4 {
        Vec4::from_slice(&self._e[4 * i..4 * i + 3])
    }

    pub fn row(&self, j: usize) -> Vec4 {
        Vec4::new(
            self._e[0 + j],
            self._e[4 + j],
            self._e[8 + j],
            self._e[12 + j],
        )
    }

    pub fn e(&self, i: usize, j: usize) -> f64 {
        self._e[i * 4 + j]
    }

    pub fn lookto(eye: Pnt3, dir: Vec3, up: Vec3) -> Mat4 {
        let f = dir.normalize();
        let s = f.cross(up).normalize();
        let u = s.cross(f);

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat4::new(
            s.x,         u.x,            -f.x,       0.0,
            s.y,         u.y,            -f.y,       0.0,
            s.z,         u.z,            -f.z,       0.0,
            -eye.dot(s), -eye.dot(u),    eye.dot(f), 1.0
        )
    }

    pub fn lookat(eye: Pnt3, at: Pnt3, up: Vec3) -> Mat4 {
        Self::lookto(eye, at - eye, up)
    }

    pub fn perspective(fovy: f64, aspect: f64, znear: f64, zfar: f64) -> Mat4 {
        let f = (fovy / 2.0).tan().recip();
        let c0r0 = f / aspect;
        let c1r1 = f;
        let c2r2 = (zfar + znear) / (znear - zfar);
        let c3r2 = (2.0 * zfar * znear) / (znear - zfar);
        #[cfg_attr(rustfmt, rustfmt_skip)]
        Mat4::new(
            c0r0, 0.0, 0.0, 0.0,
            0.0, c1r1, 0.0, 0.0,
            0.0, 0.0, c2r2, -1.0,
            0.0, 0.0, c3r2, 0.0
        )
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;
    fn mul(self, rhs: Mat4) -> Self::Output {
        let col0 = (0..3).map(|j| self.col(j) * rhs.e(0, j)).sum();
        let col1 = (0..3).map(|j| self.col(j) * rhs.e(1, j)).sum();
        let col2 = (0..3).map(|j| self.col(j) * rhs.e(2, j)).sum();
        let col3 = (0..3).map(|j| self.col(j) * rhs.e(3, j)).sum();
        Mat4::from_columns(col0, col1, col2, col3)
    }
}

impl Into<[f32; 16]> for Mat4 {
    fn into(self) -> [f32; 16] {
        self._e.map(|x| x as f32)
    }
}
