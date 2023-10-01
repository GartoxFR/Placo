use crate::model::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pos: Vec2,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(pos: Vec2, width: u32, height: u32) -> Self {
        Self { pos, width, height }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

