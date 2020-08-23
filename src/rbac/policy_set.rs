use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use bitvec::prelude::*;
use num_traits::FromPrimitive;

use crate::rbac::Policy;

#[derive(Clone, Debug)]
pub struct PolicySet<P: Policy>(HashSet<P>);

impl<P: Policy> Deref for PolicySet<P> {
    type Target = HashSet<P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Policy> DerefMut for PolicySet<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P: Policy> PolicySet<P> {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        const TO_USIZE_ERROR: &str = "Unable to convert Policy to usize";
        let mut bits = BitVec::<Msb0, u8>::new();
        if let Some(max_discriminant) = self
            .0
            .iter()
            .map(|p| p.to_usize().expect(TO_USIZE_ERROR))
            .max()
        {
            // BitVec length must % 8 = 0 to get correct zero filled bytes
            let n_bits = optimum_vec_length(max_discriminant + 1); // discriminant start from zero
            assert_eq!(n_bits % 8, 0);
            bits.resize(n_bits, false);
        };
        self.0
            .iter()
            .fold(bits, |mut bits, p| {
                let index = p.to_usize().expect(TO_USIZE_ERROR);
                bits.set(index, true);
                bits
            })
            .into_vec()
    }

    /// Parse set of policies from encoded bytes
    ///
    /// In case of identity server is separated from web server.
    /// Identity server may use a newer version of policy library which likely to add newer policies.
    /// If that policies are used on outdated web server, this function will return error result
    /// with known policies.
    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Self, Self> {
        BitSlice::<Msb0, u8>::from_slice(bytes)
            .into_iter()
            .enumerate()
            .filter(|t| *t.1)
            .try_fold(
                Self::new(),
                |mut acc, (index, _)| match FromPrimitive::from_usize(index) {
                    Some(policy) => {
                        acc.insert(policy);
                        Ok(acc)
                    }
                    None => Err(acc),
                },
            )
    }
}

impl<P: Policy> FromIterator<P> for PolicySet<P> {
    fn from_iter<I: IntoIterator<Item = P>>(iter: I) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl<P: Policy> From<Vec<P>> for PolicySet<P> {
    fn from(vec: Vec<P>) -> Self {
        Self(HashSet::from_iter(vec))
    }
}

#[inline]
fn optimum_vec_length(n: usize) -> usize {
    let bits = u8::BITS as usize;
    let rem = n % bits;
    if rem > 0 {
        n + (bits - rem)
    } else {
        n
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPolicy;

    use super::*;

    #[test]
    fn serialization() {
        let mut ps = PolicySet::new();
        ps.insert(TestPolicy::Policy2);
        ps.insert(TestPolicy::Policy1);
        let b1 = ps.to_bytes();
        let b2 = PolicySet::<TestPolicy>::parse_from_bytes(&b1)
            .unwrap()
            .to_bytes();
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_optimum_vec_length() {
        assert_eq!(optimum_vec_length(0), 0);
        assert_eq!(optimum_vec_length(1), 8);
        assert_eq!(optimum_vec_length(2), 8);
        assert_eq!(optimum_vec_length(3), 8);
        assert_eq!(optimum_vec_length(4), 8);
        assert_eq!(optimum_vec_length(5), 8);
        assert_eq!(optimum_vec_length(6), 8);
        assert_eq!(optimum_vec_length(7), 8);
        assert_eq!(optimum_vec_length(8), 8);
        assert_eq!(optimum_vec_length(9), 16);
        assert_eq!(optimum_vec_length(10), 16);
        assert_eq!(optimum_vec_length(11), 16);
        assert_eq!(optimum_vec_length(12), 16);
        assert_eq!(optimum_vec_length(13), 16);
        assert_eq!(optimum_vec_length(14), 16);
        assert_eq!(optimum_vec_length(15), 16);
        assert_eq!(optimum_vec_length(16), 16);
        assert_eq!(optimum_vec_length(17), 24);
        assert_eq!(optimum_vec_length(18), 24);
        assert_eq!(optimum_vec_length(19), 24);
        assert_eq!(optimum_vec_length(20), 24);
        assert_eq!(optimum_vec_length(21), 24);
        assert_eq!(optimum_vec_length(22), 24);
        assert_eq!(optimum_vec_length(23), 24);
        assert_eq!(optimum_vec_length(24), 24);
    }
}
