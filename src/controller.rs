use crate::message::Message;
use crate::model::plan::Plan;

use self::command::BoxedCommand;
use self::state::{ControllerState, ShapeType};

pub mod command;
pub mod state;

#[derive(Debug)]
pub struct Controller {
    plan: Plan,
    done_commands: Vec<BoxedCommand>,
    undone_commands: Vec<BoxedCommand>,
    state: ControllerState,
    scale: f32,
}

impl Default for Controller {
    fn default() -> Self {
        Self {
            plan: Default::default(),
            done_commands: Default::default(),
            undone_commands: Default::default(),
            state: Default::default(),
            scale: 20.0,
        }
    }
}

impl Controller {
    pub fn update(&mut self, message: Message) {
        match (&self.state, message) {
            (ControllerState::Idle, Message::AddRectangleButton) => {
                self.state = ControllerState::AddingShape(ShapeType::Rectangle)
            }
            (ControllerState::Idle, Message::AddCircleButton) => {
                self.state = ControllerState::AddingShape(ShapeType::Circle)
            }
            (ControllerState::Idle, Message::MoveButton) => {
                self.state = ControllerState::MovingShapes
            }
            (ControllerState::Idle, Message::Undo) => self.undo(),
            (ControllerState::Idle, Message::Redo) => self.redo(),
            (_, Message::ScaleUp) => self.scale += 5.0,
            (_, Message::ScaleDown) => {
                self.scale -= 5.0;
                self.scale = self.scale.max(5.0);
            },
            (_, Message::Cancel) => self.state = ControllerState::Idle,
            (_, Message::AddShape(shape)) => {
                let id = self.plan.generate_shape_id();
                self.do_command(command::add_shape(id, shape));
                self.state = ControllerState::Idle;
            }
            (_, Message::UpdateShape(shape_id, shape)) => {
                if let Some(old) = self.plan.get_shape(shape_id) {
                    self.do_command(command::update_shape(shape_id, old.clone(), shape));
                }
            },
            _ => {}
        }
    }

    pub fn plan(&self) -> &Plan {
        &self.plan
    }

    pub fn can_undo(&self) -> bool {
        self.idle() && !self.done_commands.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        self.idle() && !self.undone_commands.is_empty()
    }

    pub fn do_command(&mut self, command: BoxedCommand) {
        command.apply(&mut self.plan);
        // When we execute a new command, we can't redo anymore
        self.undone_commands.clear();
        self.done_commands.push(command);
    }

    pub fn undo(&mut self) {
        if let Some(command) = self.done_commands.pop() {
            command.undo(&mut self.plan);
            self.undone_commands.push(command);
        }
    }

    pub fn redo(&mut self) {
        if let Some(command) = self.undone_commands.pop() {
            command.apply(&mut self.plan);
            self.done_commands.push(command);
        }
    }

    pub fn state(&self) -> &ControllerState {
        &self.state
    }

    pub fn idle(&self) -> bool {
        matches!(self.state, ControllerState::Idle)
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}
