use iced::{Application, Command};
use iced::executor::Default as DefaultExecutor;
use iced::Theme as IcedTheme;

use crate::controller::Controller;
use crate::message::Message;
use crate::view;

#[derive(Debug, Default)]
pub struct App {
    controller: Controller
}

impl Application for App {
    type Executor = DefaultExecutor;

    type Message = Message;

    type Theme = IcedTheme;

    type Flags = ();

    fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        "Placo".into()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.controller.update(message);

        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        view::main_view(&self.controller)
    }
}
