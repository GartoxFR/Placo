use crate::model::plan::{ShapeId, Plan};
use crate::model::shape::Shape;

use super::{Command, BoxedCommand};

#[derive(Debug)]
pub struct AddShape {
    shape_id: ShapeId,
    shape: Shape,
}

pub fn add_shape(shape_id: ShapeId, shape: Shape) -> BoxedCommand {
    Box::new(AddShape { shape_id, shape })
}

impl Command for AddShape {
    fn apply(&self, plan: &mut Plan) {
        plan.add_shape(self.shape_id, self.shape.clone());
    }

    fn undo(&self, plan: &mut Plan) {
        plan.remove_shape(self.shape_id);
    }
}
