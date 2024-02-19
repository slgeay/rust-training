// https://github.com/ferrous-systems/rust-exercises/blob/main/exercise-book/src/shapes.md

use num::Num;
use std::ops::{Mul, MulAssign};

/// A shape that has an area
trait HasArea<T> {
    fn area(&self) -> T;
}

/// Describes a circle
struct Circle<T> {
    /// The radius of the circle
    radius: T
}

impl<T> Circle<T> {
    /// Creates a circle
    fn new(radius: T) -> Circle<T> {
        Circle { radius }
    }

    /// Calculates the area of the circle
    fn area(&self) -> T where T: Num + Mul<Output = T> + From<f32> + Copy {
        T::from(std::f32::consts::PI) * self.radius * self.radius
    }

    /// Changes the size of the circle
    fn scale(&mut self, factor: T) where T: Num + MulAssign {
        self.radius *= factor;
    }

    /// Destroys the circle and returns the radius
    fn destroy(self) -> T {
        self.radius
    }
    
}

impl<T> HasArea<T> for Circle<T> where T: Num + Copy + From<f32>{
    fn area(&self) -> T {
        self.area() as T
    }
}

/// Describes a square
struct Square<T> {
    /// The length of a side of the square
    side: T
}

impl<T> Square<T> {
    /// Creates a square
    fn new(side: T) -> Square<T> {
        Square { side }
    }

    /// Calculates the area of the square
    fn area(&self) -> T where T: Num + Mul<Output = T> + Copy{
        self.side * self.side
    }

    /// Changes the size of the square
    fn scale(&mut self, factor: T) where T: Num + MulAssign {
        self.side *= factor;
    }

    /// Destroys the square and returns the side length
    fn destroy(self) -> T {
        self.side
    }
}

impl<T> HasArea<T> for Square<T> where T: Num + Copy + From<f32> {
    fn area(&self) -> T {
        self.area() as T
    }
}

enum Shape<T> {
    Circle(Circle<T>),
    Square(Square<T>)
}

impl<T> HasArea<T> for Shape<T> where T: Num + Copy + From<f32> {
    fn area(&self) -> T {
        match self {
            Shape::Circle(c) => HasArea::area(c),
            Shape::Square(s) => HasArea::area(s)
        }
    }

    
}

pub fn main() {
    println!("<<< Shapes >>>");

    let c = Circle::new(1.0);
    let s = Square::new(1.0);
    let shapes = vec![Shape::Circle(c), Shape::Square(s)];
    let areas = shapes.iter().map(|s| s.area()).collect::<Vec<_>>();
    println!("{:?}", areas);    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let mut c = Circle::new(1.0);
        assert_eq!(c.area(), std::f32::consts::PI);
        c.scale(2.0);
        assert_eq!(c.area(), 4.0 * std::f32::consts::PI);
        assert_eq!(c.destroy(), 2.0);
    }

    #[test]
    fn test_square() {
        let mut s = Square::new(1.0);
        assert_eq!(s.area(), 1.0);
        s.scale(2.0);
        assert_eq!(s.area(), 4.0);
        assert_eq!(s.destroy(), 2.0);
    }

    #[test]
    fn test_shape() {
        let c = Circle::new(1.0);
        let s = Square::new(1.0);
        let shapes = vec![Shape::Circle(c), Shape::Square(s)];
        let areas = shapes.iter().map(|s| s.area()).collect::<Vec<_>>();
        assert_eq!(areas[0], std::f64::consts::PI as f32);
        assert_eq!(areas[1], 1.0);
    }

    #[test]
    fn test_f32() {
        let c = Circle::new(1.0f32);
        let s = Square::new(1.0f32);
        let shapes = vec![Shape::Circle(c), Shape::Square(s)];
        let areas = shapes.iter().map(|s| s.area()).collect::<Vec<_>>();
        assert_eq!(areas[0], std::f32::consts::PI);
        assert_eq!(areas[1], 1.0);
    }

    #[test]
    fn test_f64() {
        let c = Circle::new(1.0f64);
        let s = Square::new(1.0f64);
        let shapes = vec![Shape::Circle(c), Shape::Square(s)];
        let areas = shapes.iter().map(|s| s.area()).collect::<Vec<_>>();
        assert_eq!(areas[0], std::f32::consts::PI as f64);
        assert_eq!(areas[1], 1.0);
    }
}