use std::mem;

use iced::event::Status;
use iced::widget::canvas::{Event, Frame, Program, Stroke};
use iced::{mouse, Color, Point};

use crate::controller::state::ShapeType;
use crate::controller::Controller;
use crate::message::Message;
use crate::model::shape::{Circle, Rectangle, Shape};
use crate::model::vec2::Vec2;
use crate::view;
use crate::view::plan::screen_to_world;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Positioning,
    Sizing {
        first_point: Vec2,
        shape: Shape,
    },
}

#[derive(Debug)]
pub struct Add<'a> {
    controller: &'a Controller,
    shape_type: ShapeType,
}

impl<'a> Add<'a> {
    pub fn new(controller: &'a Controller, shape_type: ShapeType) -> Self {
        Self {
            controller,
            shape_type,
        }
    }
}

impl Program<Message> for Add<'_> {
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &iced::Renderer<iced::Theme>,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<<iced::Renderer<iced::Theme> as iced::widget::canvas::Renderer>::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        view::plan::draw_shapes(&mut frame, self.controller.plan().iter(), self.controller.scale());

        if let State::Sizing { shape, .. } = state {
            let path = view::plan::draw_shape(shape, self.controller.scale());

            frame.stroke(
                &path,
                Stroke::default().with_color(Color::BLACK).with_width(2.0),
            );
        }

        vec![frame.into_geometry()]
    }

    fn update(
        &self,
        state: &mut Self::State,
        event: Event,
        bounds: iced::Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> (Status, Option<Message>) {
        match (&mut *state, event) {
            (
                State::Positioning,
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            ) => {
                let Some(screen_cursor_pos) = cursor.position_in(bounds) else {
                        return (Status::Ignored, None);
                    };

                let world_cursor_pos = screen_to_world(screen_cursor_pos, self.controller.scale());
                *state = State::Sizing {
                    shape: generate_shape(self.shape_type, world_cursor_pos, world_cursor_pos),
                    first_point: world_cursor_pos,
                };

                (Status::Captured, None)
            }
            (
                State::Sizing {
                    first_point,
                    ref mut shape,
                },
                Event::Mouse(mouse::Event::CursorMoved {
                    position: absolute_pos,
                }),
            ) => {
                if bounds.contains(absolute_pos) {
                    let relative_pos = Point::ORIGIN + (absolute_pos - bounds.position());
                    let second_point = screen_to_world(relative_pos, self.controller.scale());

                    *shape = generate_shape(self.shape_type, *first_point, second_point);
                    (Status::Captured, None)
                } else {
                    (Status::Ignored, None)
                }
            }
            (
                State::Sizing { .. },
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
            ) => {
                if cursor.position_in(bounds).is_none() {
                    return (Status::Ignored, None);
                }
                // We replace the state with Positioning to move shape and not clone it
                // The new state does't matter because the message sent should make the
                // controller switch back to the Idle program
                let State::Sizing { shape, .. } =
                        mem::replace(state, State::Positioning) else { unreachable!() };

                (Status::Captured, Some(Message::AddShape(shape)))
            }
            (
                State::Sizing { .. },
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
            ) => (Status::Captured, Some(Message::Cancel)),
            _ => (Status::Ignored, None),
        }
    }
}

fn generate_shape(shape_type: ShapeType, first_point: Vec2, second_point: Vec2) -> Shape {
    match shape_type {
        ShapeType::Circle => generate_circle(first_point, second_point),
        ShapeType::Rectangle => generate_recangle(first_point, second_point),
    }
}

fn generate_circle(center: Vec2, second_point: Vec2) -> Shape {
    let radius = center.distance(&second_point);
    Shape::Circle(Circle::new(center, radius))
}

fn generate_recangle(first_point: Vec2, second_point: Vec2) -> Shape {
    let top_left_x = i32::min(first_point.x, second_point.x);
    let top_left_y = i32::min(first_point.y, second_point.y);

    let width = i32::abs(first_point.x - second_point.x) as u32;
    let height = i32::abs(first_point.y - second_point.y) as u32;

    Shape::Rectangle(Rectangle::new(
        (top_left_x, top_left_y).into(),
        width,
        height,
    ))
}
