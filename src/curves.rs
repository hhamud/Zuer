use crate::{Fe, PrimeField};
use ruint::aliases::U256;
use ruint::uint;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BN254;
impl PrimeField for BN254 {
    type Number = U256;
    const PRIME: U256 =
        uint!(0x30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47_U256);
    const A: U256 = uint!(0_U256);
    const B: U256 = uint!(3_U256);
    const NAME: &'static str = "BN254";
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmallPrime;
impl PrimeField for SmallPrime {
    type Number = u64;
    const PRIME: u64 = 101;
    const A: u64 = 98; // -3
    const B: u64 = 3;
    const NAME: &'static str = "F101";
}

impl From<U256> for Fe<BN254> {
    fn from(v: U256) -> Self {
        super::Fe::<BN254>::new(v)
    }
}

impl From<u64> for Fe<BN254> {
    fn from(v: u64) -> Self {
        Fe::<BN254>::new(U256::from(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ruint::aliases::U256;
    use ruint::uint;

    #[test]
    fn test_conversions() {
        let a: Fe<BN254> = 5u64.into();
        let b: Fe<BN254> = uint!(42_U256).into();
        assert_eq!(a.value(), U256::from(5));
        assert_eq!(b.value(), U256::from(42));

        let x: Fe<SmallPrime> = 17u64.into();
        assert_eq!(x.value(), 17);
    }

    #[test]
    fn test_addition() {
        let x: Fe<SmallPrime> = 94u64.into();
        let y: Fe<SmallPrime> = 8.into();
        let z = x + y;

        assert_eq!(z.value(), 1);
    }

    #[test]
    fn test_addition_overflow() {
        let x: Fe<SmallPrime> = 100u64.into();
        let y: Fe<SmallPrime> = 1.into();
        let z = x + y;
        assert_eq!(z.value(), 0);

        let x: Fe<SmallPrime> = 100u64.into();
        let y: Fe<SmallPrime> = 2.into();
        let z = x + y;
        assert_eq!(z.value(), 1);
    }

    #[test]
    fn test_subtraction() {
        let x: Fe<SmallPrime> = 10u64.into();
        let y: Fe<SmallPrime> = 7.into();
        let z = x - y;
        assert_eq!(z.value(), 3);
    }

    #[test]
    fn test_subtraction_underflow() {
        let x: Fe<SmallPrime> = 10u64.into();
        let y: Fe<SmallPrime> = 12.into();
        let z = x - y;
        assert_eq!(z.value(), 99);
    }
}
