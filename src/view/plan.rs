
use iced::widget::canvas::{Frame, Path, Program};
use iced::widget::Canvas;
use iced::{widget, Color, Length, Point, Size};

use crate::controller::state::ControllerState;
use crate::controller::Controller;
use crate::message::Message;
use crate::model::plan::ShapeId;
use crate::model::shape::{Circle, Rectangle, Shape};
use crate::model::vec2::Vec2;

use super::Element;

mod program;

/// Match the different pattern with the condition to select the program
/// for a canvas and then format it with the given format function
macro_rules! canvas_select_program_and_format {
    ($cond:expr => {$($pattern:pat => $program: expr),+} $closure: ident) => {
        match $cond {
            $($pattern => {
                let canvas = widget::canvas($program);
                $closure(canvas).into()
            },)*
        }
    };
}

pub fn plan_view(controller: &Controller) -> Element {
    canvas_select_program_and_format! {
        controller.state() => {
            ControllerState::Idle => program::Idle::new(controller),
            ControllerState::AddingShape(shape_type) => program::Add::new(controller, *shape_type),
            ControllerState::MovingShapes => program::Move::new(controller)
        }
        format_canvas
    }
}

/// This function is used to parameter the canvas for all possible
/// Program
fn format_canvas<P: Program<Message>>(canvas: Canvas<P, Message>) -> Canvas<P, Message> {
    canvas.width(Length::Fill).height(Length::Fill)
}

fn draw_shapes<'a>(
    frame: &mut Frame,
    shapes: impl Iterator<Item = &'a (ShapeId, Shape)>,
    scale: f32,
) {
    for (_id, shape) in shapes {
        let path = draw_shape(shape, scale);
        frame.fill(&path, Color::BLACK);
    }
}

fn draw_shape(shape: &Shape, scale: f32) -> Path {
    match shape {
        Shape::Circle(circle) => draw_circle(circle, scale),
        Shape::Rectangle(rectangle) => draw_rectangle(rectangle, scale),
    }
}

fn draw_circle(circle: &Circle, scale: f32) -> Path {
    let center = world_to_screen(circle.pos(), scale);
    let radius = circle.radius() as f32 * scale;
    Path::circle(center, radius)
}

fn draw_rectangle(rectangle: &Rectangle, scale: f32) -> Path {
    let top_left = world_to_screen(rectangle.pos(), scale);

    let size = Size::new(
        rectangle.width() as f32 * scale,
        rectangle.height() as f32 * scale,
    );

    Path::rectangle(top_left, size)
}

fn screen_to_world(screen_pos: Point, scale: f32) -> Vec2 {
    Vec2::new((screen_pos.x / scale) as i32, (screen_pos.y / scale) as i32)
}

fn world_to_screen(world_pos: Vec2, scale: f32) -> Point {
    Point::new(world_pos.x as f32 * scale, world_pos.y as f32 * scale)
}
