use std::mem;

use iced::event::Status;
use iced::widget::canvas::{Event, Frame, Program, Stroke};
use iced::{mouse, Color, Point};

use crate::controller::Controller;
use crate::message::Message;
use crate::model::plan::ShapeId;
use crate::model::shape::Shape;
use crate::view;
use crate::view::plan::screen_to_world;

#[derive(Debug, Default)]
pub enum State {
    #[default]
    Chosing,
    Moving {
        shape: Shape,
        shape_id: ShapeId,
    },
}

#[derive(Debug)]
pub struct Move<'a> {
    controller: &'a Controller,
}

impl<'a> Move<'a> {
    pub fn new(controller: &'a Controller) -> Self {
        Self { controller }
    }
}

impl Program<Message> for Move<'_> {
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

        let filter_id = match state {
            State::Chosing => None,
            State::Moving { shape_id, .. } => Some(shape_id),
        };
        let shapes = self
            .controller
            .plan()
            .iter()
            .filter(|(id, _)| Some(id) != filter_id);
        view::plan::draw_shapes(&mut frame, shapes, self.controller.scale());

        if let State::Moving { shape, .. } = state {
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
            (State::Chosing, Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))) => {
                let Some(screen_cursor_pos) = cursor.position_in(bounds) else {
                        return (Status::Ignored, None);
                    };

                let world_cursor_pos = screen_to_world(screen_cursor_pos, self.controller.scale());
                if let Some((shape_id, shape)) =
                    self.controller.plan().get_shape_at(&world_cursor_pos)
                {
                    let mut shape = shape.clone();
                    shape.move_to(world_cursor_pos);
                    *state = State::Moving {
                        shape,
                        shape_id: *shape_id,
                    }
                }

                (Status::Captured, None)
            }
            (
                State::Moving { ref mut shape, shape_id },
                Event::Mouse(mouse::Event::CursorMoved {
                    position: absolute_pos,
                }),
            ) => {
                if bounds.contains(absolute_pos) {
                    let relative_pos = Point::ORIGIN + (absolute_pos - bounds.position());
                    let new_pos = screen_to_world(relative_pos, self.controller.scale());
                    let mut new_shape = shape.clone();
                    new_shape.move_to(new_pos);

                    if self.controller.plan().is_disjoint(&new_shape, Some(*shape_id)) {
                        *shape = new_shape;
                    }

                    (Status::Captured, None)
                } else {
                    (Status::Ignored, None)
                }
            }
            (
                State::Moving { .. },
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
            ) => {
                let State::Moving { shape, shape_id } =
                        mem::replace(state, State::Chosing) else { unreachable!() };

                (
                    Status::Captured,
                    Some(Message::UpdateShape(shape_id, shape)),
                )
            }
            (_, Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right))) => {
                (Status::Captured, Some(Message::Cancel))
            }
            _ => (Status::Ignored, None),
        }
    }
}
