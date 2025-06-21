use crate::{Fe, PrimeField};
use num_traits::{One, Pow};
use std::ops::{Add, Mul};

pub struct Point<P: PrimeField> {
    pub x: Fe<P>,
    pub y: Fe<P>,
}

impl<P: PrimeField> Point<P> {
    pub fn new(x: Fe<P>, y: Fe<P>) -> Self {
        Self { x, y }
    }
}

impl<P: PrimeField> Add for Point<P> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let two = Fe::new(P::Number::one() + P::Number::one());
        let gradient = (&rhs.y - &self.y) / (&rhs.x - &self.x);
        let x = (&gradient).pow(two) - &self.x - &rhs.x;
        let y = gradient * (&self.x - &x) - self.y;

        Self::new(x, y)
    }
}
