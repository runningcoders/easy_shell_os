use crate::math::tow_d::{Point, Vector};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Rectangle {
    left_up: Point,
    right_up: Point,
    right_down: Point,
    left_down: Point,
}

impl Rectangle {
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        let (left_1, left_2) = (p1.more_left(&p2), p3.more_left(&p4));
        let (right_1, right_2) = (p1.more_right(&p2), p3.more_right(&p4));

        if left_1.is_vertical_with(left_2) {
            Self {
                left_up: left_1.more_up(left_2).clone(),
                right_up: right_1.more_up(right_2).clone(),
                right_down: right_1.more_down(right_2).clone(),
                left_down: left_1.more_down(left_2).clone(),
            }
        } else if left_1.is_left_of(left_2) {
            Self {
                left_up: left_1.more_up(right_1).clone(),
                right_up: left_2.more_up(right_2).clone(),
                right_down: left_2.more_down(right_2).clone(),
                left_down: left_1.more_down(right_1).clone(),
            }
        } else {
            Self {
                left_up: left_2.more_up(right_2).clone(),
                right_up: left_1.more_up(right_1).clone(),
                right_down: left_1.more_down(right_1).clone(),
                left_down: left_2.more_down(right_2).clone(),
            }
        }
    }

    pub fn with_diagonal_point(start: Point, end: Point) -> Self {
        let (start_clone, end_clone) = (start.clone(), end.clone());
        Self::new(
            start,
            Point::new(end_clone.x, start_clone.y),
            end,
            Point::new(start_clone.x, end_clone.y),
        )
    }

    pub fn with_diagonal(start: Point, v: &Vector) -> Self {
        let mut end = start.clone();
        end.add(v);
        Self::with_diagonal_point(start, end)
    }

    pub fn fix_point(&mut self, p: &Point) {
        if self.check_point_in(p) {
            return;
        }

        (self.left_up.x, self.left_up.y) = (self.left_up.x.min(p.x), self.left_up.y.min(p.y));
        (self.right_up.x, self.right_up.y) = (self.right_up.x.max(p.x), self.right_up.y.min(p.y));
        (self.right_down.x, self.right_down.y) =
            (self.right_down.x.max(p.x), self.right_down.y.max(p.y));
        (self.left_down.x, self.left_down.y) =
            (self.left_down.x.min(p.x), self.left_down.y.max(p.y));
    }

    pub fn check_point_in(&self, p: &Point) -> bool {
        !self.check_point_out(p)
    }

    pub fn check_point_out(&self, p: &Point) -> bool {
        p.is_left_of(self.left_point())
            || p.is_up_of(self.up_point())
            || p.is_right_of(self.right_point())
            || p.is_down_of(self.down_point())
    }

    pub fn left_point(&self) -> &Point {
        &self.left_up
    }

    pub fn up_point(&self) -> &Point {
        &self.left_up
    }

    pub fn right_point(&self) -> &Point {
        &self.right_down
    }

    pub fn down_point(&self) -> &Point {
        &self.right_down
    }

    pub fn check_hit(&self, other: &Self) -> bool {
        !self.check_no_hit(other)
    }

    pub fn check_no_hit(&self, other: &Self) -> bool {
        self.right_point().is_left_of(other.left_point())
            || self.down_point().is_up_of(other.up_point())
            || self.left_point().is_right_of(other.right_point())
            || self.up_point().is_down_of(other.down_point())
    }
}

impl FromStr for Rectangle {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.replace(|ch: char| ch.is_whitespace(), "");

        let coords: Vec<&str> = s
            .trim_matches(|p| p == '[' || p == ']')
            .split("),(")
            .collect();

        Ok(Rectangle::new(
            coords[0].parse()?,
            coords[1].parse()?,
            coords[2].parse()?,
            coords[3].parse()?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::fraction;
    use std::borrow::Borrow;

    #[test]
    fn rectangle_new() {
        let (dx, dy) = ("2".parse().unwrap(), "1".parse().unwrap());
        let (p1, p2, p3, p4) = (
            Point::default(),
            Point::new(dx, fraction::ZERO),
            Point::new(fraction::ZERO, dy),
            Point::new(dx, dy),
        );

        assert_eq!(
            Rectangle::new(p1.clone(), p2.clone(), p3.clone(), p4.clone(),),
            Rectangle::new(p2, p1, p4, p3)
        );
    }

    #[test]
    fn rectangle_with_diagonal() {
        let (dx, dy) = ("2".parse().unwrap(), "1".parse().unwrap());

        assert_eq!(
            Rectangle::with_diagonal(Point::default(), Vector::new(dx, dy).borrow()),
            Rectangle::new(
                Point::default(),
                Point::new(dx, fraction::ZERO),
                Point::new(fraction::ZERO, dy),
                Point::new(dx, dy)
            )
        );
    }

    #[test]
    fn rectangle_from_str() {
        assert_eq!(
            Rectangle::from_str("[(0, 0), (1, 0), (1, 1), (0, 1)]").unwrap(),
            Rectangle::with_diagonal(
                Point::default(),
                Vector::new(fraction::ONE, fraction::ONE).borrow()
            )
        )
    }
}
