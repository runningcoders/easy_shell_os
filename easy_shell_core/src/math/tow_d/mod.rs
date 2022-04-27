mod rectangle;

use crate::math::fraction::Fraction;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Point {
    x: Fraction,
    y: Fraction,
}

impl Point {
    pub fn new(x: Fraction, y: Fraction) -> Self {
        Self { x, y }
    }

    pub fn is_left_of(&self, other: &Self) -> bool {
        self.x < other.x
    }
    pub fn more_left<'a>(&'a self, other: &'a Self) -> &'a Self {
        if other.is_left_of(self) {
            other
        } else {
            self
        }
    }

    pub fn is_up_of(&self, other: &Self) -> bool {
        self.y < other.y
    }
    pub fn more_up<'a>(&'a self, other: &'a Self) -> &'a Self {
        if other.is_up_of(self) {
            other
        } else {
            self
        }
    }

    pub fn is_right_of(&self, other: &Self) -> bool {
        other.x < self.x
    }
    pub fn more_right<'a>(&'a self, other: &'a Self) -> &'a Self {
        if other.is_right_of(self) {
            other
        } else {
            self
        }
    }

    pub fn is_down_of(&self, other: &Self) -> bool {
        other.y < self.y
    }
    pub fn more_down<'a>(&'a self, other: &'a Self) -> &'a Self {
        if other.is_down_of(self) {
            other
        } else {
            self
        }
    }

    pub fn is_horizontal_with(&self, other: &Self) -> bool {
        self.y == other.y
    }

    pub fn is_vertical_with(&self, other: &Self) -> bool {
        self.x == other.x
    }

    pub fn add(&mut self, v: &Vector) {
        self.x += v.dx;
        self.y += v.dy;
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse()?;

        let mut p = Self::default();
        p.add(&v);

        Ok(p)
    }
}

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Vector {
    dx: Fraction,
    dy: Fraction,
}

impl Vector {
    pub fn new(dx: Fraction, dy: Fraction) -> Self {
        Self { dx, dy }
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "v({},{})", self.dx, self.dy)
    }
}

impl FromStr for Vector {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s
            .trim()
            .trim_matches(&['v', '(', ')'][..])
            .split(',')
            .collect();

        Ok(Self::new(coords[0].parse()?, coords[1].parse()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn point_more_left() {
        let (a, b) = (
            Point::default(),
            Point::new("0".parse().unwrap(), "1".parse().unwrap()),
        );
        assert_eq!(a.borrow(), a.more_left(&b));
    }
}
