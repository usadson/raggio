// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::ops::{Mul, Range};

use num_traits::{Num, Signed};

pub fn min<T>(lhs: T, rhs: T) -> T
        where T: Copy + PartialOrd {
    if lhs < rhs {
        lhs
    } else {
        rhs
    }
}

pub fn max<T>(lhs: T, rhs: T) -> T
        where T: Copy + PartialOrd {
    if lhs > rhs {
        lhs
    } else {
        rhs
    }
}

// Can't do packing, see https://github.com/rust-lang/rust/issues/82523 :(
// #[repr(packed)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Vector2<T> where T: Num + Copy {
    pub x: T,
    pub y: T,
}

pub type Vector2f = Vector2<f32>;

impl<T> Vector2<T> where T: Num + Copy {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

#[repr(packed)]
#[derive(Copy, Clone, Default, Debug, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3 {

    /// The cross product of the two vectors.
    pub fn cross(&self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    /// Sizzle the x and y components off this 3D vector.
    pub fn xy(&self) -> Vector2f {
        Vector2 { x: self.x, y: self.y }
    }

}

/// The cross product of the two vectors.
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    /// The cross product of the two vectors.
    fn mul(self, rhs: Vector3) -> Self::Output {
        self.cross(rhs)
    }
}

pub struct Rectangle2D<T> where T: Num + Copy {
    lefttopmost: Vector2<T>,
    rightbottommost: Vector2<T>,
}

impl<T> Rectangle2D<T> where T: Num + Copy {
    /// Get the X coordinate
    pub fn x(&self) -> T {
        self.lefttopmost.x
    }

    /// Get the range of the X coordinate
    pub fn x_range(&self) -> Range<T> {
        self.lefttopmost.x..self.rightbottommost.x
    }

    /// Get the Y coordinate
    pub fn y(&self) -> T {
        self.lefttopmost.y
    }

    /// Get the range of the Y coordinate
    pub fn y_range(&self) -> Range<T> {
        self.lefttopmost.y..self.rightbottommost.y
    }
}

pub struct Triangle2D<T>(pub Vector2<T>, pub Vector2<T>, pub Vector2<T>) where T: Num + Copy + PartialOrd;

impl<T> Triangle2D<T> where T: Num + Signed + Copy + PartialOrd {

    /// Gets the area of the triangle using [Heron's Formula](https://en.wikipedia.org/wiki/Heron%27s_formula).
    pub fn area(&self) -> T {
        (
            (self.1.x - self.0.x) * (self.2.y - self.0.y)
                -
            (self.2.x - self.0.x) * (self.1.y - self.0.y)
        ).abs()
    }

    /// Creates a rectangle that encapsulates the triangle.
    pub fn encapsulating_rectangle(&self) -> Rectangle2D<T> {
        Rectangle2D {
            lefttopmost: Vector2::new(self.min_x(), self.min_y()),
            rightbottommost: Vector2::new(self.max_x(), self.max_y()),
        }
    }

    /// Tests if the point is inside the triangle.
    pub fn hit_test(&self, point: Vector2<T>) -> bool {
        let area = self.area();

        let a1 = Triangle2D(point, self.0, self.1).area();
        let a2 = Triangle2D(point, self.1, self.2).area();
        let a3 = Triangle2D(point, self.2, self.0).area();

        a1 + a2 + a3 == area
    }

    /// Finds the highest x coordinate.
    pub fn max_x(&self) -> T {
        max(self.0.x, max(self.1.x, self.2.x))
    }

    /// Finds the lowest y coordinate.
    pub fn max_y(&self) -> T {
        max(self.0.y, max(self.1.y, self.2.y))
    }

    /// Finds the lowest x coordinate.
    pub fn min_x(&self) -> T {
        min(self.0.x, min(self.1.x, self.2.x))
    }

    /// Finds the lowest y coordinate.
    pub fn min_y(&self) -> T {
        min(self.0.y, min(self.1.y, self.2.y))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle2d_bounds() {
        let triangle = Triangle2D(Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0),
            Vector2::new(2.0, 2.0));
        assert_eq!(triangle.min_x(), 0.0);
        assert_eq!(triangle.min_y(), 0.0);
        assert_eq!(triangle.max_x(), 2.0);
        assert_eq!(triangle.max_y(), 2.0);

        let triangle = Triangle2D(Vector2::new(-2.0, -8.0), Vector2::new(525.7, 732.5),
            Vector2::new(1.0, 1.0));
        assert_eq!(triangle.min_x(), -2.0);
        assert_eq!(triangle.min_y(), -8.0);
        assert_eq!(triangle.max_x(), 525.7);
        assert_eq!(triangle.max_y(), 732.5);
    }

    #[test]
    fn triangle2d_encapsulating_rectangle() {
        let triangle = Triangle2D(Vector2::new(-612.0, 62.4), Vector2::new(-4882.72, -5.0),
            Vector2::new(100.3, -41005.5));
        let rectangle = triangle.encapsulating_rectangle();
        assert_eq!(rectangle.x(), -4882.72);
        assert_eq!(rectangle.y(), -41005.5);
        assert_eq!(rectangle.x_range(), -4882.72..100.3);
        assert_eq!(rectangle.y_range(), -41005.5..62.4);
    }
}
