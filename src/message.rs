use crate::model::plan::ShapeId;
use crate::model::shape::Shape;

#[derive(Debug, Clone)]
pub enum Message {
    AddRectangleButton,
    AddCircleButton,
    MoveButton,
    AddShape(Shape),
    UpdateShape(ShapeId, Shape),
    ScaleUp,
    ScaleDown,
    Cancel,
    Undo,
    Redo
}
