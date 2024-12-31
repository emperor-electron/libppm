use std::fmt::Display;

/// Coordinate on a Cartesian plane.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

/// LineCoordinates is a tuple struct around two Coordinate structs that represent the endpoints of a
/// line.
#[derive(Debug, PartialEq)]
pub struct LineCoordinates(pub Coordinate, pub Coordinate);

pub struct TriangleCoordinates(pub Coordinate, pub Coordinate, pub Coordinate);

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Coordinate(x:{}, y:{})", self.x, self.y)
    }
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    /// This function returns a tuple that represents the change in the x &
    /// y coordinates in self with respect to the 'other' coordinate.
    ///
    /// # Example
    ///
    /// ```
    /// use libppm::coordinate::Coordinate;
    ///
    /// let a = Coordinate::new(0, 0);
    /// let b = Coordinate::new(1, 1);
    /// let (dx, dy) = a.delta_wrt(&b);
    ///
    /// assert_eq!(dx, 1);
    /// assert_eq!(dy, 1);
    ///
    /// let (dx, dy) = b.delta_wrt(&a);
    ///
    /// assert_eq!(dx, -1);
    /// assert_eq!(dy, -1);
    /// ```
    pub fn delta_wrt(&self, other: &Coordinate) -> (i32, i32) {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        (dx, dy)
    }
}

impl LineCoordinates {
    pub fn new(a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> Self {
        LineCoordinates(Coordinate::new(a_x, a_y), Coordinate::new(b_x, b_y))
    }

    /// Calculates the slope of the two Coordinates enclosed within.
    ///
    /// # Example
    ///
    /// ```
    /// use libppm::coordinate::LineCoordinates;
    ///
    /// let line = LineCoordinates::new(0, 0, 5, 5);
    /// assert_eq!(line.slope(), 1.0);
    /// ```
    pub fn slope(&self) -> f32 {
        if self.1.x - self.0.x == 0 {
            f32::INFINITY
        } else {
            (self.1.y - self.0.y) as f32 / (self.1.x - self.0.x) as f32
        }
    }

    pub fn ensure_x_lr(&self) -> Self {
        if self.0.x <= self.1.x {
            LineCoordinates::new(self.0.x, self.0.y, self.1.x, self.1.y)
        } else {
            LineCoordinates::new(self.1.x, self.1.y, self.0.x, self.0.y)
        }
    }

    pub fn ensure_y_lr(&self) -> Self {
        if self.0.y <= self.1.y {
            LineCoordinates::new(self.0.x, self.0.y, self.1.x, self.1.y)
        } else {
            LineCoordinates::new(self.1.x, self.1.y, self.0.x, self.0.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_x_lr() {
        let line_coords = LineCoordinates::new(0, 0, 1, 1).ensure_x_lr();

        assert_eq!(line_coords, LineCoordinates::new(0, 0, 1, 1));

        let line_coords = LineCoordinates::new(1, 1, 0, 0).ensure_x_lr();

        assert_eq!(line_coords, LineCoordinates::new(0, 0, 1, 1));
    }

    #[test]
    fn test_ensure_y_lr() {
        let line_coords = LineCoordinates::new(0, 0, 1, 1).ensure_y_lr();

        assert_eq!(line_coords, LineCoordinates::new(0, 0, 1, 1));

        let line_coords = LineCoordinates::new(1, 1, 0, 0).ensure_y_lr();

        assert_eq!(line_coords, LineCoordinates::new(0, 0, 1, 1));
    }
}
