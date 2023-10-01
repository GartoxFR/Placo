use crate::model::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Circle {
    pos: Vec2,
    radius: u32,
}

impl Circle {
    pub fn new(pos: Vec2, radius: u32) -> Self {
        Self { pos, radius }
    }

    pub fn pos(&self) -> Vec2 {
        self.pos
    }

    pub fn radius(&self) -> u32 {
        self.radius
    }
}

