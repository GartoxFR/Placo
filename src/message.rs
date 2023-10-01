use crate::model::shape::Shape;

#[derive(Debug, Clone)]
pub enum Message {
    AddRectangleButton,
    AddCircleButton,
    AddShape(Shape),
    ScaleUp,
    ScaleDown,
    Cancel,
    Undo,
    Redo
}
