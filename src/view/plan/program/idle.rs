use iced::widget::canvas::{Program, Frame};

use crate::controller::Controller;
use crate::message::Message;
use crate::view;

pub struct Idle<'a> {
    controller: &'a Controller,
}

impl<'a> Idle<'a> {
    pub fn new(controller: &'a Controller) -> Self {
        Self { controller }
    }
}

impl Program<Message> for Idle<'_> {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &iced::Renderer<iced::Theme>,
        _theme: &iced::Theme,
        bounds: iced::Rectangle,
        _cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<<iced::Renderer<iced::Theme> as iced::widget::canvas::Renderer>::Geometry> {

        let mut frame = Frame::new(renderer, bounds.size());

        view::plan::draw_shapes(&mut frame, self.controller.plan().iter(), self.controller.scale());

        vec![frame.into_geometry()]
    }
}
