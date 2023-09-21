use crate::area::Area;
use std::fmt::{Display, Formatter};

pub struct Circle {
    pub x: f64,
    pub y: f64,
    pub radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        let value = self.radius * self.radius * std::f64::consts::PI;

        println!("traits::Circle::area");
        println!("{}", value);
        println!();

        return value;
    }
}

impl Default for Circle {
    fn default() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            radius: 0.0,
        };
    }
}

impl Display for Circle {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(
            formatter,
            "Circle(x{} y{} r{})",
            self.x, self.y, self.radius
        );
    }
}
