use std::fmt::{Display, Formatter};
use crate::area::Area;

pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        let value = self.width * self.height;

        println!("traits::Rectangle::area");
        println!("{}", value);
        println!();

        return value;
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        return Self {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        };
    }
}

impl Display for Rectangle {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        return write!(formatter, "Rectangle(x{} y{} w{} h{})", self.x, self.y, self.width, self.height);
    }
}