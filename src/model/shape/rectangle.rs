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

    pub fn move_to(&mut self, to: Vec2) {
        self.pos = to;
    }

    pub fn contains(&self, point: &Vec2) -> bool {
        point.x >= self.pos.x
            && point.x <= self.pos.x + self.width as i32
            && point.y >= self.pos.y
            && point.y <= self.pos.y + self.height as i32
    }
}
