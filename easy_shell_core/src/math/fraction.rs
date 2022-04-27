use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;

lazy_static! {
    pub static ref ZERO: Fraction = {
        Fraction {
            numerator: 0,
            denominator: 1,
        }
    };
    pub static ref ONE: Fraction = {
        Fraction {
            numerator: 1,
            denominator: 1,
        }
    };
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fraction {
    numerator: i64,
    denominator: i64,
}

impl Fraction {
    pub fn new(numerator: i64, denominator: i64) -> Self {
        let mut f = Self {
            numerator,
            denominator,
        };
        f.fix();
        f
    }

    pub fn is_zero(&self) -> bool {
        self.numerator == 0
    }

    pub fn is_int(&self) -> bool {
        self.denominator == 1
    }

    pub fn is_positive(&self) -> bool {
        self.numerator > 0
    }

    pub fn is_negative(&self) -> bool {
        self.numerator < 0
    }

    pub fn opposite(&self) -> Self {
        Self::new(-self.numerator, self.denominator)
    }

    pub fn reverse(&self) -> Self {
        if self.is_zero() {
            let _ = self.denominator / self.numerator;
        }
        Self::new(self.denominator, self.numerator)
    }

    fn fix(&mut self) {
        if self.denominator < 0 {
            self.numerator = -self.numerator;
            self.denominator = -self.denominator;
        }

        let g = gcd(self.numerator.abs() as u64, self.denominator as u64) as i64;
        if g != 0 {
            self.numerator /= g;
            self.denominator /= g;
        }
    }
}

impl Default for Fraction {
    fn default() -> Self {
        *ZERO
    }
}

impl Add for Fraction {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let numerator = self.numerator * rhs.denominator + rhs.numerator * self.denominator;
        let denominator = self.denominator * rhs.denominator;
        Self::new(numerator, denominator)
    }
}

impl AddAssign for Fraction {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs);
    }
}

impl Sub for Fraction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs.opposite())
    }
}

impl SubAssign for Fraction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.sub(rhs);
    }
}

impl Mul for Fraction {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl MulAssign for Fraction {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs);
    }
}

impl Div for Fraction {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self.mul(rhs.reverse())
    }
}

impl DivAssign for Fraction {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

impl PartialOrd<Self> for Fraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Fraction {
    fn cmp(&self, other: &Self) -> Ordering {
        match *self - *other {
            x if x.is_positive() => Ordering::Greater,
            x if x.is_negative() => Ordering::Less,
            _ => Ordering::Equal,
        }
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ if self.is_zero() => {
                write!(f, "0")
            }
            _ if self.is_int() => {
                write!(f, "{}", self.numerator)
            }
            _ => {
                write!(f, "{}/{}", self.numerator, self.denominator)
            }
        }
    }
}

impl FromStr for Fraction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Self::default());
        }

        let s: Vec<i64> = s.split('/').map(|v| v.trim().parse().unwrap()).collect();
        if s.is_empty() {
            return Ok(Self::default());
        }

        Ok(s.iter()
            .skip(1)
            .fold(Self::new(s[0], 1), |acc, v| acc / Self::from(*v)))
    }
}

impl From<i64> for Fraction {
    fn from(v: i64) -> Self {
        Self {
            numerator: v,
            denominator: 1,
        }
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    let (a, b) = (a.max(b), a.min(b));
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        assert_eq!(Fraction::new(1, 2), Fraction::new(2, 4))
    }

    #[test]
    fn display() {
        assert_eq!("0", format!("{}", *ZERO));
        assert_eq!("1", format!("{}", *ONE));
        assert_eq!("1/2", format!("{}", Fraction::new(1, 2)));
    }

    #[test]
    fn from_str() {
        let a: Fraction = "0".parse().unwrap();
        assert_eq!(a, *ZERO);
        let a = Fraction::from_str("1").unwrap();
        assert_eq!(a, *ONE);
        let a = Fraction::from_str("-1 / 2").unwrap();
        assert_eq!(a, Fraction::new(-1, 2));
    }

    #[test]
    fn add() {
        let a = Fraction::from_str("-1/2").unwrap();
        let a1 = Fraction::from_str("1/2").unwrap();
        assert_eq!(a + a1, *ZERO);
    }

    #[test]
    fn sub() {
        let a = Fraction::from_str("1/3").unwrap();
        let a1 = Fraction::from_str("1/3").unwrap();
        assert_eq!(a - a1, *ZERO);
    }

    #[test]
    fn mul() {
        let a = Fraction::from_str("1/2").unwrap();
        let a1 = Fraction::from_str("2").unwrap();
        assert_eq!(a * a1, *ONE);
    }

    #[test]
    fn div() {
        let a = Fraction::from_str("1/2").unwrap();
        let a1 = Fraction::from_str("1/2").unwrap();
        assert_eq!(a / a1, *ONE);
    }

    #[test]
    fn ord() {
        let a = Fraction::from_str("2").unwrap();
        let b = Fraction::from_str("1").unwrap();
        assert!(a > b);
        assert!(b < a);
        assert_eq!(b, *ONE);
    }
}
