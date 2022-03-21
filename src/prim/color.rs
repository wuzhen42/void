#[derive(Clone, Copy)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGB {
    pub fn black() -> RGB {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn red() -> RGB {
        RGB {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }
}
