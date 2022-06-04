use std::collections::BTreeSet;
use std::iter::FromIterator;

use crate::rbac::traits::Role;
use crate::rbac::PermissionSet;

#[derive(Default)]
pub struct RoleSet<R: Role>(BTreeSet<R>);

impl<R: Role> RoleSet<R> {
    pub fn new() -> Self {
        Self(Default::default())
    }

    pub fn to_permission_set(&self) -> PermissionSet<R::Permission> {
        self.0
            .iter()
            .map(Role::permissions)
            .fold(PermissionSet::new(), |mut acc, permissions| {
                acc.extend(permissions.iter().copied());
                acc
            })
    }
}

impl<R: Role> FromIterator<R> for RoleSet<R> {
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        Self(BTreeSet::from_iter(iter))
    }
}

impl<R: Role> From<Vec<R>> for RoleSet<R> {
    fn from(v: Vec<R>) -> Self {
        Self::from_iter(v)
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestRole;

    use super::*;

    #[test]
    fn test_role_set_to_permission_set() {
        let mut rs: RoleSet<TestRole> = RoleSet::new();
        rs.0.insert(TestRole::Role0);
        assert_eq!(rs.to_permission_set().inner().len(), 2);
    }
}
