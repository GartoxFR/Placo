pub mod add;
pub mod update;

use core::fmt;

use crate::model::plan::Plan;

pub use add::add_shape;
pub use update::update_shape;

pub trait Command: fmt::Debug {
    fn apply(&self, plan: &mut Plan);
    fn undo(&self, plan: &mut Plan);
}

pub type BoxedCommand = Box<dyn Command + 'static>;


