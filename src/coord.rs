use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, other: Coord) {
        *self = Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}
