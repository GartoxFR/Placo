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

    pub fn move_to(&mut self, to: Vec2) {
        self.pos = to;
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        point.distance(&self.pos) <= self.radius() 
    }
}

