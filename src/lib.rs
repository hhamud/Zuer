use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
    ops::{Add, Div, Mul, Rem, Sub},
};

pub mod curves;

pub trait PrimeField {
    type Number: Copy
        + Eq
        + PartialOrd
        + Debug
        + Display
        + Add<Output = Self::Number>
        + Sub<Output = Self::Number>
        + Mul<Output = Self::Number>
        + Div<Output = Self::Number>
        + Rem<Output = Self::Number>;

    const PRIME: Self::Number;
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

impl<P: PrimeField> Sub for Fe<P> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new((self.value + P::PRIME) - rhs.value)
    }
}

impl<P: PrimeField<Number = u64>> From<u64> for Fe<P> {
    fn from(v: u64) -> Self {
        Self::new(P::Number::from(v))
    }
}
