use crate::Fe;
use crate::PrimeField;
use ruint::aliases::U256;
use ruint::uint;

#[derive(Clone, Copy, Debug)]
pub struct BN254;
impl PrimeField for BN254 {
    type Number = U256;
    const PRIME: U256 =
        uint!(0x30644E72E131A029B85045B68181585D97816A916871CA8D3C208C16D87CFD47_U256);
    const NAME: &'static str = "BN254";
}

#[derive(Clone, Copy, Debug)]
pub struct SmallPrime;
impl PrimeField for SmallPrime {
    type Number = u64;
    const PRIME: u64 = 101;
    const NAME: &'static str = "F101";
}

impl From<U256> for super::Fe<BN254> {
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
    fn conversions() {
        let a: Fe<BN254> = 5u64.into();
        let b: Fe<BN254> = uint!(42_U256).into();
        assert_eq!(a.value(), U256::from(5));
        assert_eq!(b.value(), U256::from(42));

        let x: Fe<SmallPrime> = 17u64.into();
        assert_eq!(x.value(), 17);
    }
}
