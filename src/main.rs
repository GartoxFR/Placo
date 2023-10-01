use iced::{Application, Settings};

mod app;
mod message;
mod view;
mod model;
mod controller;
mod utils;

fn main() {
    app::App::run(Settings::default()).unwrap();
}
