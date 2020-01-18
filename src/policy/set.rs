use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use bitvec::prelude::*;
use num_traits::cast::ToPrimitive;
use num_traits::FromPrimitive;

use crate::error::Error;
use crate::policy::PolicyCount;

#[derive(Debug)]
pub struct PolicySet<P>(HashSet<P>)
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount;

impl<P> Deref for PolicySet<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    type Target = HashSet<P>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P> DerefMut for PolicySet<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<P> PolicySet<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bit_vec = BitVec::<Msb0, u8>::new();
        // BitVec size must % 8 = 0 to get correct bytes
        let new_len = {
            let total = <P as PolicyCount>::count();
            let bits = u8::BITS as usize;
            let rem = total % bits;
            if rem > 0 {
                total + (bits - rem)
            } else {
                total
            }
        };
        bit_vec.resize(new_len, false);
        self.0.iter()
            .fold(bit_vec.into_boxed_bitslice(), |mut bits, p| {
                let index = p.to_usize().expect("cannot convert to usize");
                bits.set(index, true);
                bits
            })
            .as_slice()
            .to_vec()
    }

    pub fn from_unparsed_input(input: &str) -> Result<Self, (usize, Self)> {
        input
            .trim_start_matches("[")
            .trim_end_matches("]")
            .split(",")
            .filter_map(|s: &str| s.trim().parse::<usize>().ok())
            .try_fold(Self::new(), |mut policies, id| {
                if let Some(p) = P::from_usize(id) {
                    policies.insert(p);
                    Ok(policies)
                } else {
                    Err((id, policies))
                }
            })
    }

    pub fn from_bytes(buf: &[u8]) -> Result<Self, Error> {
        BitSlice::<Msb0, u8>::from_slice(buf)
            .into_iter()
            .take(P::count()) // Prevent out of date server to crash
            .enumerate()
            .filter(|t| *t.1)
            .try_fold(Self::new(), |mut acc, (index, _)| {
                match FromPrimitive::from_usize(index) {
                    Some(policy) => {
                        acc.insert(policy);
                        Ok(acc)
                    }
                    None => Err(Error::BadPolicyEncoding)
                }
            })
    }
}

impl<P> FromIterator<P> for PolicySet<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn from_iter<I: IntoIterator<Item=P>>(iter: I) -> Self {
        let mut set = PolicySet::<P>::new();
        for p in iter.into_iter() {
            set.insert(p);
        }
        set
    }
}

impl<P> From<Vec<P>> for PolicySet<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn from(vec: Vec<P>) -> Self {
        vec.into_iter().collect()
    }
}

impl<T> From<HashSet<T>> for PolicySet<T>
    where T: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn from(set: HashSet<T>) -> Self {
        PolicySet(set)
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::tests::TestPolicy;

    use super::*;

    #[test]
    fn serialization() {
        let mut ps = PolicySet::new();
        ps.insert(TestPolicy::Policy2);
        ps.insert(TestPolicy::Policy1);
        let b1 = ps.to_bytes();
        let b2 = PolicySet::<TestPolicy>::from_bytes(&b1).unwrap().to_bytes();
        assert_eq!(b1, b2);
    }
}