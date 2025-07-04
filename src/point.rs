use crate::{Fe, PrimeField};
use num_traits::{One, Pow};
use std::ops::{Add, AddAssign, Mul};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point<P: PrimeField> {
    pub x: Option<Fe<P>>,
    pub y: Option<Fe<P>>,
}

impl<P: PrimeField> Point<P> {
    pub fn new(x: Fe<P>, y: Fe<P>) -> Self {
        let two = P::Number::one() + P::Number::one();
        let three = two + P::Number::one();
        if y.pow(Fe::new(two)) != x.pow(Fe::new(three)) + Fe::new(P::A) * x + Fe::new(P::B) {
            panic!("Point is not on the curve");
        }
        Self {
            x: Some(x),
            y: Some(y),
        }
    }

    pub fn inf() -> Self {
        Self { x: None, y: None }
    }

    pub fn is_inf(&self) -> bool {
        self.x.is_none()
    }
}

impl<P: PrimeField> Add for Point<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.is_inf() {
            return rhs;
        }
        if rhs.is_inf() {
            return self;
        }

        let x1 = self.x.as_ref().unwrap();
        let y1 = self.y.as_ref().unwrap();
        let x2 = rhs.x.as_ref().unwrap();
        let y2 = rhs.y.as_ref().unwrap();

        let two = Fe::new(P::Number::one() + P::Number::one());

        let gradient = if x1 == x2 {
            if y1 != y2 {
                // Point inverse, e.g., P + (-P) = Inf
                return Self::inf();
            }
            // Point doubling, P + P = 2P
            let three = Fe::new(P::Number::one() + P::Number::one() + P::Number::one());
            (three * x1.pow(two) + Fe::new(P::A)) / (two * y1)
        } else {
            // Point addition, P + Q
            (y2 - y1) / (x2 - x1)
        };

        let x3 = gradient.pow(two) - x1 - x2;
        let y3 = gradient * (x1 - &x3) - y1;

        Self::new(x3, y3)
    }
}

impl<P: PrimeField> AddAssign for Point<P> {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<P: PrimeField> Mul<u128> for Point<P> {
    type Output = Point<P>;

    fn mul(self, rhs: u128) -> Self::Output {
        if rhs == 0 || self.is_inf() {
            return Point::inf();
        }

        let mut res = Point::inf();
        let add = self;

        // Left-to-right binary exponentiation (double and add)
        let first_bit_pos = u128::BITS - rhs.leading_zeros();
        for i in (0..first_bit_pos).rev() {
            res += res; // Double
            if (rhs >> i) & 1 == 1 {
                res += add; // Add
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::SmallPrime;

    #[test]
    fn test_point_addition() {
        // Using a curve y^2 = x^3 - 3x + 3 over F_101
        // Let P = (1, 1)
        let p1 = Point::<SmallPrime>::new(1u64.into(), 1u64.into());
        // Let Q = (6, 10)
        let p2 = Point::<SmallPrime>::new(6u64.into(), 10u64.into());

        // P + Q = (73, 31)
        let p3 = p1 + p2;
        assert_eq!(p3.x.unwrap().value(), 73);
        assert_eq!(p3.y.unwrap().value(), 31);
    }

    #[test]
    fn test_point_doubling() {
        // Using a curve y^2 = x^3 - 3x + 3 over F_101
        // Let P = (1, 1)
        let p1 = Point::<SmallPrime>::new(1u64.into(), 1u64.into());

        // 2P = (99, 100)
        let p2 = p1 + p1;
        assert_eq!(p2.x.unwrap().value(), 99);
        assert_eq!(p2.y.unwrap().value(), 100);
    }

    #[test]
    fn test_scalar_multiplication() {
        // Using a curve y^2 = x^3 - 3x + 3 over F_101
        // Let P = (1, 1)
        let p = Point::<SmallPrime>::new(1u64.into(), 1u64.into());

        // 3P = (80, 81)
        let p3 = p * 3;
        assert_eq!(p3.x.unwrap().value(), 80);
        assert_eq!(p3.y.unwrap().value(), 81);
    }
}
