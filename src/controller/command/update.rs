use crate::model::plan::{ShapeId, Plan};
use crate::model::shape::Shape;

use super::{BoxedCommand, Command};

#[derive(Debug)]
struct UpdateShape {
    new: Shape,
    old: Shape,
    shape_id: ShapeId,
}

pub fn update_shape(shape_id: ShapeId, old: Shape, new: Shape) -> BoxedCommand {
    Box::new(UpdateShape { shape_id, old, new })
}

impl Command for UpdateShape {
    fn apply(&self, plan: &mut Plan) {
        plan.replace_shape(self.shape_id, self.new.clone());
    }

    fn undo(&self, plan: &mut Plan) {
        plan.replace_shape(self.shape_id, self.old.clone());
    }
}
