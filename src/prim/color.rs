#[derive(Clone, Copy)]
pub struct RGB {
    pub r: f64,
    pub g: f64,
    pub b: f64,
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

    pub fn green() -> RGB {
        RGB {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> RGB {
        RGB {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }
}
