use num_traits::Zero;
use std::ops::{Add, Mul};

use crate::{Fe, PrimeField};

pub const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Polynomial<P: PrimeField, const N: usize> {
    pub coefficients: [Fe<P>; N],
}

impl<P: PrimeField, const N: usize> Polynomial<P, N> {
    pub fn new(coefficients: [Fe<P>; N]) -> Self {
        Self { coefficients }
    }

    pub fn degree(&self) -> usize {
        N.saturating_sub(1)
    }

    /// Evaluates the polynomial at a given point `x` using Horner's method.
    pub fn evaluate(&self, x: Fe<P>) -> Fe<P> {
        self.coefficients
            .iter()
            .rfold(Fe::<P>::new(P::Number::zero()), |acc, &coeff| {
                acc * x + coeff
            })
    }
}

impl<P: PrimeField, const N: usize, const M: usize> Add<Polynomial<P, M>> for Polynomial<P, N>
where
    [(); max(N, M)]:,
{
    type Output = Polynomial<P, { max(N, M) }>;

    fn add(self, rhs: Polynomial<P, M>) -> Self::Output {
        let mut result_coeffs = [Fe::<P>::new(P::Number::zero()); max(N, M)];
        let min_len = if N < M { N } else { M };

        for i in 0..min_len {
            result_coeffs[i] = self.coefficients[i] + rhs.coefficients[i];
        }

        if N > M {
            for i in M..N {
                result_coeffs[i] = self.coefficients[i];
            }
        } else {
            for i in N..M {
                result_coeffs[i] = rhs.coefficients[i];
            }
        }
        Polynomial::new(result_coeffs)
    }
}

impl<P: PrimeField, const N: usize, const M: usize> Mul<Polynomial<P, M>> for Polynomial<P, N>
where
    [(); N + M - 1]:,
{
    type Output = Polynomial<P, { N + M - 1 }>;

    fn mul(self, rhs: Polynomial<P, M>) -> Self::Output {
        let mut result_coeffs = [Fe::<P>::new(P::Number::zero()); N + M - 1];

        for i in 0..N {
            for j in 0..M {
                result_coeffs[i + j] =
                    result_coeffs[i + j] + self.coefficients[i] * rhs.coefficients[j];
            }
        }
        Polynomial::new(result_coeffs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curves::SmallPrime;

    fn fe(v: u64) -> Fe<SmallPrime> {
        v.into()
    }

    #[test]
    fn test_addition_to_zero_polynomial() {
        // P(x) = 5x^2 + 2x + 1
        let p = Polynomial::new([fe(1), fe(2), fe(5)]);
        // Z(x) = 0
        let z = Polynomial::new([fe(0), fe(0)]);

        let result = p + z;
        assert_eq!(result.coefficients, [fe(1), fe(2), fe(5)]);
    }

    #[test]
    fn test_addition_is_commutative() {
        // A(x) = 3x^2 + 10
        let a = Polynomial::new([fe(10), fe(0), fe(3)]);
        // B(x) = 4x + 2
        let b = Polynomial::new([fe(2), fe(4)]);

        let expected = Polynomial::new([fe(12), fe(4), fe(3)]);

        let result1 = a + b;
        assert_eq!(result1, expected);

        let result2 = b + a;
        assert_eq!(result2, expected);
    }

    #[test]
    fn test_addition_with_internal_zeros() {
        // A(x) = x^3 + 1
        let a = Polynomial::new([fe(1), fe(0), fe(0), fe(1)]);
        // B(x) = x^2 + 1
        let b = Polynomial::new([fe(1), fe(0), fe(1)]);

        // Expected result: x^3 + x^2 + 2
        let expected = Polynomial::new([fe(2), fe(0), fe(1), fe(1)]);

        let result = a + b;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_multiplication() {
        // A(x) = 2x + 1
        let a = Polynomial::new([fe(1), fe(2)]);
        // B(x) = 3x + 2
        let b = Polynomial::new([fe(2), fe(3)]);

        // Expected: (2x+1)*(3x+2) = 6x^2 + 4x + 3x + 2 = 6x^2 + 7x + 2
        let expected = Polynomial::new([fe(2), fe(7), fe(6)]);

        let result = a * b;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluation() {
        // P(x) = 2x^2 + 3x + 4
        let p = Polynomial::new([fe(4), fe(3), fe(2)]);

        // Evaluate at x = 2
        // P(2) = 2*(2^2) + 3*2 + 4 = 2*4 + 6 + 4 = 8 + 6 + 4 = 18
        let result = p.evaluate(fe(2));
        assert_eq!(result, fe(18));

        // Evaluate at x = 3
        // P(3) = 2*(3^2) + 3*3 + 4 = 2*9 + 9 + 4 = 18 + 9 + 4 = 31
        let result2 = p.evaluate(fe(3));
        assert_eq!(result2, fe(31));
    }
}
