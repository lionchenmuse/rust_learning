use crate::area::Area;

pub struct Triangle {
    base: f64,
    height: f64
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Triangle {
        Triangle { base, height }
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}