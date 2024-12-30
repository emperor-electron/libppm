use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub struct LineCoordinates(pub Coordinate, pub Coordinate);
pub struct TriangleCoordinates(pub Coordinate, pub Coordinate, pub Coordinate);

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate(x:{}, y:{})", self.x, self.y)
    }
}
