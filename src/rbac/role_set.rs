use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

use crate::rbac::PolicySet;
use crate::rbac::traits::Role;

pub struct RoleSet<R: Role>(HashSet<R>);

impl<R: Role> RoleSet<R> {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn to_policy_set(&self) -> PolicySet<R::Policy> {
        self.0
            .iter()
            .flat_map(Role::as_policy_set_ref)
            .fold(PolicySet::new(), |mut acc, ps| {
                acc.extend(ps.iter());
                acc
            })
    }
}

impl<R: Role> FromIterator<R> for RoleSet<R> {
    fn from_iter<I: IntoIterator<Item=R>>(iter: I) -> Self {
        Self(HashSet::from_iter(iter))
    }
}

impl<R: Role> Deref for RoleSet<R> {
    type Target = HashSet<R>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Role> DerefMut for RoleSet<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestRole;

    use super::*;

    #[test]
    fn test_role_set_to_policy_set() {
        let mut rs: RoleSet<TestRole> = RoleSet::new();
        rs.insert(TestRole::Role0);
        assert_eq!(rs.to_policy_set().len(), 2);
    }
}
