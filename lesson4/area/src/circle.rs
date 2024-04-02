use std::f64::consts::PI;

use crate::area::Area;

pub struct Circle {
    r: f64
}

impl Circle {
    pub fn new(r: f64) -> Circle {
        Circle { r }
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.r * self.r
    }
}