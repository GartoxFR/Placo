use iced::widget::{button, column, container, row};
use iced::Length;

use crate::controller::Controller;
use crate::message::Message;
use crate::utils::message_resolver::*;

use self::plan::plan_view;

mod plan;

const SPACING: u16 = 5;
const PADDING: u16 = 10;

const BUTTONS: [(&str, MessageResolver); 7] = [
    ("Add rectangle", message_if!(Message::AddRectangleButton, Controller::idle)),
    ("Add circle", message_if!(Message::AddCircleButton, Controller::idle)),
    ("Move", message_if!(Message::MoveButton, Controller::idle)),
    ("Scale up", message_if!(Message::ScaleUp, Controller::idle)),
    ("Scale down", message_if!(Message::ScaleDown, Controller::idle)),
    ("Undo", message_if!(Message::Undo, Controller::can_undo)),
    ("Redo", message_if!(Message::Redo, Controller::can_redo)),
];

type Element<'a> = iced::Element<'a, Message, iced::Renderer<iced::Theme>>;

pub fn main_view(controller: &Controller) -> Element {
    let buttons = button_panel(controller);
    let plan_view = plan_view(controller);

    let content = row![buttons, plan_view].spacing(SPACING).padding(PADDING);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn button_panel(controller: &Controller) -> Element {
    let buttons = BUTTONS
        .iter()
        .map(|(name, message_resolver)| {
            button(*name)
                .on_press_maybe(message_resolver(controller).clone())
                .style(iced::theme::Button::Positive)
                .width(Length::Fill)
                .into()
        })
        .collect();

    container(column(buttons).width(Length::Fixed(150.0)).spacing(SPACING))
        .style(iced::theme::Container::Box)
        .padding(PADDING)
        .height(Length::Fill)
        .into()
}
