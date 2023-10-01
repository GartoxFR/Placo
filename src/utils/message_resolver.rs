//! This module exposes utility to define convenient notations
//! to express the `Message` sent by a component based on the 
//! state of the `Controller`
//!
//! ```rust
//! use crate::utils::message_resolver::*;
//!
//! const BUTTONS: [(&str, MessageResolver); 2] = [
//!     ("Add circle", always!(Message::AddCircle)),
//!     ("Undo", message_if!(Message::Undo, Controller::can_undo))
//! ]
//! ```

use crate::controller::Controller;
use crate::message::Message;

pub type MessageResolver = fn(&Controller) -> Option<Message>;


#[allow(unused)]
macro_rules! always {
    ($message: expr) => {
        |_| Some($message)
    };
}

macro_rules! message_if {
    ($message: expr, $pred: expr) => {
        |controller| {
            let filter: fn(&Controller) -> bool = $pred;
            Some($message).filter(|_| filter(controller))
        }
    };
}

#[allow(unused)]
pub(crate) use always;
pub(crate) use message_if;
