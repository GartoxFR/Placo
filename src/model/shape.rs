pub mod rectangle;
pub use rectangle::*;

pub mod circle;
pub use circle::*;

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl From<Rectangle> for Shape {
    fn from(value: Rectangle) -> Self {
        Self::Rectangle(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Self::Circle(value)
    }
}
