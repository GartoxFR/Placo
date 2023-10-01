#[derive(Debug, Default, Clone)]
pub enum ControllerState {
    #[default]
    Idle,
    AddingShape(ShapeType)
}

#[derive(Debug, Clone, Copy)]
pub enum ShapeType {
    Circle,
    Rectangle
}
