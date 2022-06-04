use std::iter::FromIterator;

use Predicate::*;

use crate::rbac::Permission;
use crate::rbac::PermissionSet;

#[derive(Clone)]
pub enum Predicate<P: Permission> {
    Nil,
    Contains(P),
    Any(Box<[P]>),
    All(Box<[P]>),
}

impl<P: Permission> Predicate<P> {
    pub fn satisfy(&self, permissions: &PermissionSet<P>) -> bool {
        let set = permissions.inner();
        match self {
            Nil => true,
            Contains(permission) => set.contains(permission),
            Any(slice) => slice.iter().any(|permission| set.contains(permission)),
            All(slice) => slice.iter().all(|permission| set.contains(permission)),
        }
    }

    pub fn contains(permission: P) -> Self {
        Contains(permission)
    }

    pub fn all<T: IntoIterator<Item = P>>(iter: T) -> Self {
        All(Vec::from_iter(iter).into_boxed_slice())
    }

    pub fn any<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Any(Vec::from_iter(iter).into_boxed_slice())
    }
}

impl<P: Permission> From<P> for Predicate<P> {
    fn from(p: P) -> Self {
        Contains(p)
    }
}

impl<P: Permission> AsRef<Predicate<P>> for Predicate<P> {
    fn as_ref(&self) -> &Predicate<P> {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPermission::{self, *};

    use super::*;

    type TestPredicate = Predicate<TestPermission>;

    #[test]
    fn no_should_always_satisfy() {
        assert!(TestPredicate::Nil.satisfy(&[].into()));
        assert!(TestPredicate::Nil.satisfy(&[Permission1].into()));
        assert!(TestPredicate::Nil.satisfy(&[Permission1, Permission2].into()));
    }

    #[test]
    fn contains_should_satisfy() {
        assert!(TestPredicate::contains(Permission1).satisfy(&[Permission1].into()));
        assert!(TestPredicate::contains(Permission1).satisfy(&[Permission1, Permission2].into()));
    }

    #[test]
    fn contains_should_not_satisfy() {
        assert!(!TestPredicate::contains(Permission1).satisfy(&[].into()));
        assert!(!TestPredicate::contains(Permission1).satisfy(&[Permission2].into()));
    }

    #[test]
    fn any_should_satisfy() {
        let check_ps: PermissionSet<_> = [Permission1, Permission2].into();
        assert!(TestPredicate::any([Permission1]).satisfy(&check_ps));
        assert!(TestPredicate::any([Permission2]).satisfy(&check_ps));
        assert!(TestPredicate::any([Permission1, Permission2]).satisfy(&check_ps));
        assert!(TestPredicate::any([Permission1, Permission2, Permission3]).satisfy(&check_ps));
    }

    #[test]
    fn any_should_not_satisfy() {
        assert!(!TestPredicate::any([]).satisfy(&[].into()));
        assert!(!TestPredicate::any([]).satisfy(&[Permission1].into()));
        assert!(!TestPredicate::any([]).satisfy(&[Permission1, Permission2].into()));

        assert!(!TestPredicate::any([Permission1]).satisfy(&[].into()));
        assert!(!TestPredicate::any([Permission1, Permission2]).satisfy(&[].into()));

        assert!(!TestPredicate::any([Permission1]).satisfy(&[Permission2].into()));
        assert!(!TestPredicate::any([Permission1]).satisfy(&[Permission2, Permission3].into()));
    }

    #[test]
    fn all_should_satisfy() {
        assert!(TestPredicate::all([Permission1]).satisfy(&[Permission1].into()));
        assert!(TestPredicate::all([Permission1, Permission2])
            .satisfy(&[Permission1, Permission2].into()));
        assert!(TestPredicate::all([Permission1, Permission2])
            .satisfy(&[Permission1, Permission2, Permission3].into()));

        assert!(TestPredicate::all([]).satisfy(&[].into()));
        assert!(TestPredicate::all([]).satisfy(&[Permission1].into()));
        assert!(TestPredicate::all([]).satisfy(&[Permission1, Permission2].into()));
    }

    #[test]
    fn all_should_not_satisfy() {
        assert!(!TestPredicate::all([Permission1]).satisfy(&[].into()));
        assert!(!TestPredicate::all([Permission1, Permission2]).satisfy(&[].into()));

        assert!(!TestPredicate::all([Permission1, Permission2]).satisfy(&[Permission1].into()));
        assert!(!TestPredicate::all([Permission1, Permission2]).satisfy(&[Permission2].into()));
        assert!(!TestPredicate::all([Permission1, Permission2])
            .satisfy(&[Permission2, Permission3].into()));
    }
}
