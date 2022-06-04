use std::collections::BTreeSet;
use std::fmt::{self, Debug};
use std::iter::FromIterator;

use bitvec::prelude::*;
use num_traits::FromPrimitive;
use tracing::trace;

use crate::rbac::Permission;

#[derive(Clone)]
pub struct PermissionSet<T>(BTreeSet<T>);

impl<T: Debug> Debug for PermissionSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<P: Permission> Default for PermissionSet<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Permission> PermissionSet<P> {
    pub fn new() -> Self {
        Self(BTreeSet::new())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        const TO_USIZE_ERROR: &str = "Unable to convert Permission to usize";
        let mut bits = BitVec::<u8, Msb0>::new();
        if let Some(max_permission_id) = self
            .0
            .iter()
            .max()
            .map(|p| p.to_usize().expect(TO_USIZE_ERROR))
        {
            // Reserve space for every possible permission.
            let len = max_permission_id + 1;
            trace!(
                "Max permission id: {}, new length: {}",
                max_permission_id,
                len
            );
            bits.resize(len, false);

            for p in self.0.iter() {
                let index = p.to_usize().expect(TO_USIZE_ERROR);
                bits.set(index, true);
            }
        }
        // We explicitly initialize all bits of vector in order to get correct zero filled bytes.
        bits.set_uninitialized(false);
        bits.into_vec()
    }

    /// Parse set of permissions from encoded bytes
    ///
    /// In case of identity server is separated from web server.
    /// Identity server may use a newer version of permission library which likely to add newer permissions.
    /// If that permissions are used on outdated web server, this function will return error result
    /// with known permissions.
    pub fn parse_from_bytes(bytes: &[u8]) -> Result<Self, Self> {
        bytes
            .view_bits::<Msb0>()
            .into_iter()
            .enumerate()
            .filter_map(|(index, bit)| bit.then(|| index))
            .try_fold(Self::new(), |mut acc, index| {
                if let Some(permission) = <P as FromPrimitive>::from_usize(index) {
                    acc.0.insert(permission);
                    Ok(acc)
                } else {
                    Err(acc)
                }
            })
    }

    pub fn iter(&self) -> Iter<'_, P> {
        Iter {
            iter: self.0.iter(),
        }
    }

    pub fn from_slice(src: &[P]) -> Self {
        src.iter().copied().collect()
    }

    pub(crate) fn inner(&self) -> &BTreeSet<P> {
        &self.0
    }
}

impl<T: Ord> FromIterator<T> for PermissionSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self(BTreeSet::from_iter(iter))
    }
}

impl<T: Ord> From<Vec<T>> for PermissionSet<T> {
    fn from(vec: Vec<T>) -> Self {
        Self::from_iter(vec)
    }
}

impl<T: Ord, const N: usize> From<[T; N]> for PermissionSet<T> {
    fn from(vec: [T; N]) -> Self {
        Self::from_iter(vec)
    }
}

impl<A: Ord> Extend<A> for PermissionSet<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

pub struct Iter<'a, T> {
    iter: std::collections::btree_set::Iter<'a, T>,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPermission;

    use super::*;

    #[test]
    fn serialization() {
        let ps = PermissionSet::from(vec![
            TestPermission::Permission2,
            TestPermission::Permission1,
        ]);
        let b1 = ps.to_bytes();
        let b2 = PermissionSet::<TestPermission>::parse_from_bytes(&b1)
            .unwrap()
            .to_bytes();
        assert_eq!(b1, b2);
    }
}
