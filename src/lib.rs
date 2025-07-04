#![feature(generic_const_exprs)]

use num_traits::{One, Pow, WrappingMul, Zero};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, BitAnd, Div, Mul, Rem, RemAssign, ShrAssign, Sub},
};

pub mod curves;
pub mod point;
pub mod polynomial;

fn modpow<P: PrimeField>(mut base: P::Number, mut exp: P::Number) -> P::Number {
    let one = P::Number::one();
    let mut acc = one % P::PRIME;
    base %= P::PRIME;
    while exp > P::Number::zero() {
        if exp & one == one {
            acc = acc.wrapping_mul(&base) % P::PRIME;
        }
        base = base.wrapping_mul(&base) % P::PRIME;
        exp >>= one;
    }
    acc
}

pub trait PrimeField: Eq + Clone + Copy {
    type Number: Copy
        + Eq
        + PartialOrd
        + Debug
        + Display
        + Add<Output = Self::Number>
        + Sub<Output = Self::Number>
        + Mul<Output = Self::Number>
        + Div<Output = Self::Number>
        + Rem<Output = Self::Number>
        + RemAssign
        + BitAnd<Output = Self::Number>
        + ShrAssign
        + WrappingMul
        + Zero
        + One;

    const PRIME: Self::Number;
    const A: Self::Number;
    const B: Self::Number;
    const NAME: &'static str;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fe<P: PrimeField> {
    value: P::Number,
    _phant: PhantomData<P>,
}

impl<P: PrimeField> Fe<P> {
    pub fn new(v: P::Number) -> Self {
        Self {
            value: v % P::PRIME,
            _phant: PhantomData,
        }
    }
    pub fn value(&self) -> P::Number {
        self.value
    }

    pub fn inv(&self) -> Option<P::Number> {
        let two = P::Number::one() + P::Number::one();
        if self.value.is_zero() {
            return None;
        }
        Some(modpow::<P>(self.value, P::PRIME - two))
    }
}

impl<P: PrimeField> Add for Fe<P> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.value + rhs.value)
    }
}

impl<P: PrimeField> Mul for Fe<P> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(self.value * rhs.value)
    }
}

impl<'a, 'b, P: PrimeField> Mul<&'b Fe<P>> for &'a Fe<P> {
    type Output = Fe<P>;

    fn mul(self, rhs: &'b Fe<P>) -> Self::Output {
        Fe::<P>::new(self.value * rhs.value)
    }
}

impl<'a, P: PrimeField> Mul<Fe<P>> for &'a Fe<P> {
    type Output = Fe<P>;

    fn mul(self, rhs: Fe<P>) -> Self::Output {
        Fe::<P>::new(self.value * rhs.value)
    }
}

impl<'a, P: PrimeField> Mul<&'a Fe<P>> for Fe<P> {
    type Output = Fe<P>;

    fn mul(self, rhs: &'a Fe<P>) -> Self::Output {
        Fe::<P>::new(self.value * rhs.value)
    }
}

impl<P: PrimeField> Sub for Fe<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new((self.value + P::PRIME) - rhs.value)
    }
}

impl<'a, P: PrimeField> Sub<&'a Fe<P>> for Fe<P> {
    type Output = Self;
    fn sub(self, rhs: &'a Fe<P>) -> Self {
        Self::new((self.value + P::PRIME) - rhs.value)
    }
}

impl<'a, 'b, P: PrimeField> Sub<&'b Fe<P>> for &'a Fe<P> {
    type Output = Fe<P>;

    fn sub(self, rhs: &'b Fe<P>) -> Self::Output {
        Fe::<P>::new((self.value + P::PRIME) - rhs.value)
    }
}

impl<P: PrimeField> Div for Fe<P> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.value * rhs.inv().unwrap())
    }
}

impl<'a, 'b, P: PrimeField> Div<&'b Fe<P>> for &'a Fe<P> {
    type Output = Fe<P>;

    fn div(self, rhs: &'b Fe<P>) -> Self::Output {
        Fe::<P>::new(self.value * rhs.inv().unwrap())
    }
}

impl<P: PrimeField<Number = u64>> From<u64> for Fe<P> {
    fn from(v: u64) -> Self {
        Self::new(P::Number::from(v))
    }
}

impl<P: PrimeField> Pow<Self> for Fe<P> {
    type Output = Self;

    fn pow(self, rhs: Self) -> Self::Output {
        Self::new(modpow::<P>(self.value, rhs.value))
    }
}

impl<'a, P: PrimeField> Pow<Fe<P>> for &'a Fe<P> {
    type Output = Fe<P>;

    fn pow(self, rhs: Fe<P>) -> Self::Output {
        Fe::<P>::new(modpow::<P>(self.value, rhs.value))
    }
}
