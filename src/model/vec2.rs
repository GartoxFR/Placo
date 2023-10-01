use std::ops::{Add, Neg, Sub, AddAssign, SubAssign};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl Vec2 {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, rhs: &Self) -> u32 {
        let diff = *self - *rhs;
        f32::sqrt((diff.x * diff.x + diff.y * diff.y) as f32) as u32
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        let (x, y) = value;
        Self { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec2_add() {
        let a = Vec2::new(1, 2);
        let mut b = Vec2::new(2, 4);

        let c = a + b;

        b += a;


        assert_eq!(c, (3, 6).into());
        assert_eq!(b, (3, 6).into());
    }

    #[test]
    fn vec2_sub() {
        let a = Vec2::new(1, 2);
        let mut b = Vec2::new(2, 4);

        let c = a - b;

        b -= a;

        assert_eq!(c, (-1, -2).into());
        assert_eq!(b, (1, 2).into());
    }

    #[test]
    fn vec2_neg() {
        let a = Vec2::new(2, 4);
        assert_eq!(-a, (-2, -4).into());
    }
}
