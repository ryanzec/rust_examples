use crate::area::Area;
use crate::circle::Circle;
use crate::rectangle::Rectangle;

mod area;
mod circle;
mod rectangle;

fn main() {
    let mut circle = Circle::default();

    circle.radius = 10.0;

    let mut rectangle = Rectangle::default();

    rectangle.width = 10.0;
    rectangle.height = 10.0;

    circle.area();
    rectangle.area();

    println!("{}", circle);
    println!("{}", rectangle);
}
