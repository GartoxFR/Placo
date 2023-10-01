pub mod rectangle;
pub use rectangle::*;

pub mod circle;
pub use circle::*;

#[derive(Debug, Clone)]
pub enum Shape {
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Shape {
    pub fn is_disjoint(&self, other: &Shape) -> bool {
        match (self, other) {
            (Shape::Circle(circle1), Shape::Circle(circle2)) => {
                Self::is_disjoint_circles(circle1, circle2)
            }
            (Shape::Rectangle(rect1), Shape::Rectangle(rect2)) => {
                Self::is_disjoint_rects(rect1, rect2)
            }
            (Shape::Circle(circle), Shape::Rectangle(rect))
            | (Shape::Rectangle(rect), Shape::Circle(circle)) => {
                Self::is_disjoint_rect_circle(rect, circle)
            }
        }
    }

    fn is_disjoint_rects(rect1: &Rectangle, rect2: &Rectangle) -> bool {
        let x_overlap = i32::min(
            rect1.pos().x + rect1.width() as i32,
            rect2.pos().x + rect2.width() as i32,
        ) - i32::max(rect1.pos().x, rect2.pos().x);

        let y_overlap = i32::min(
            rect1.pos().y + rect1.height() as i32,
            rect2.pos().y + rect2.height() as i32,
        ) - i32::max(rect1.pos().y, rect2.pos().y);

        (x_overlap <= 0) || (y_overlap <= 0)
    }

    fn is_disjoint_circles(circle1: &Circle, circle2: &Circle) -> bool {
        let dist = circle1.pos().distance(&circle2.pos());

        dist >= circle1.radius() + circle2.radius()
    }

    fn is_disjoint_rect_circle(rect: &Rectangle, circle: &Circle) -> bool {
        let nearest_x = i32::clamp(
            circle.pos().x,
            rect.pos().x,
            rect.pos().x + rect.width() as i32,
        );

        let nearest_y = i32::clamp(
            circle.pos().y,
            rect.pos().y,
            rect.pos().y + rect.height() as i32,
        );

        let nearest_point = (nearest_x, nearest_y).into();

        circle.pos().distance(&nearest_point) >= circle.radius()
    }
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn disjoint_rect_rect_1() {
        let r1: Shape = Rectangle::new((4, 5).into(), 2, 4).into();
        let r2: Shape = Rectangle::new((10, 12).into(), 1, 3).into();

        assert!(r1.is_disjoint(&r2));
        assert!(r2.is_disjoint(&r1));
    }

    #[test]
    fn disjoint_rect_rect_2() {
        let r1: Shape = Rectangle::new((4, 5).into(), 12, 14).into();
        let r2: Shape = Rectangle::new((10, 12).into(), 1, 3).into();

        assert!(!r1.is_disjoint(&r2));
        assert!(!r2.is_disjoint(&r1));
    }

    #[test]
    fn disjoint_rect_circle_1() {
        let r1: Shape = Circle::new((4, 5).into(), 2).into();
        let r2: Shape = Rectangle::new((10, 12).into(), 3, 4).into();

        assert!(r1.is_disjoint(&r2));
        assert!(r2.is_disjoint(&r1));
    }

    #[test]
    fn disjoint_rect_circle_2() {
        let r1: Shape = Circle::new((4, 5).into(), 3).into();
        let r2: Shape = Rectangle::new((6, 5).into(), 3, 4).into();

        assert!(!r1.is_disjoint(&r2));
        assert!(!r2.is_disjoint(&r1));
    }

    #[test]
    fn disjoint_circle_circle_1() {
        let r1: Shape = Circle::new((4, 5).into(), 2).into();
        let r2: Shape = Circle::new((10, 12).into(), 3).into();

        assert!(r1.is_disjoint(&r2));
        assert!(r2.is_disjoint(&r1));
    }

    #[test]
    fn disjoint_circle_circle_2() {
        let r1: Shape = Circle::new((4, 5).into(), 2).into();
        let r2: Shape = Circle::new((9, 5).into(), 4).into();

        assert!(!r1.is_disjoint(&r2));
        assert!(!r2.is_disjoint(&r1));
    }
}
