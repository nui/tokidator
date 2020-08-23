use std::iter::FromIterator;

use PolicyCond::*;

use crate::rbac::Policy;
use crate::rbac::PolicySet;

pub enum PolicyCond<P: Policy> {
    NoCheck,
    Contains(P),
    Any(PolicySet<P>),
    All(PolicySet<P>),
}

impl<P: Policy> PolicyCond<P> {
    pub fn satisfy(&self, policies: &PolicySet<P>) -> bool {
        match self {
            NoCheck => true,
            Contains(policy) => policies.contains(policy),
            Any(set) => set.iter().any(|p| policies.contains(p)),
            All(set) => !set.is_empty() && set.iter().all(|p| policies.contains(p)),
        }
    }

    pub fn contains(policy: P) -> Self {
        Contains(policy)
    }

    pub fn all<T: IntoIterator<Item = P>>(iter: T) -> Self {
        All(PolicySet::from_iter(iter))
    }

    pub fn any<T: IntoIterator<Item = P>>(iter: T) -> Self {
        Any(PolicySet::from_iter(iter))
    }
}

impl<P: Policy> From<P> for PolicyCond<P> {
    fn from(p: P) -> Self {
        Contains(p)
    }
}

#[cfg(test)]
mod tests {
    use crate::rbac::test_helpers::TestPolicy::{self, *};

    use super::*;

    #[test]
    fn no_check_should_satisfy() {
        assert!(NoCheck.satisfy(&PolicySet::<TestPolicy>::new()));
        assert!(NoCheck.satisfy(&vec![Policy1].into()));
        assert!(NoCheck.satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_satisfy() {
        assert!(PolicyCond::contains(Policy1).satisfy(&vec![Policy1].into()));
        assert!(PolicyCond::contains(Policy1).satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_not_satisfy() {
        assert!(!PolicyCond::contains(Policy1).satisfy(&vec![].into()));
        assert!(!PolicyCond::contains(Policy1).satisfy(&vec![Policy2].into()));
    }

    #[test]
    fn any_should_satisfy() {
        let check_ps: PolicySet<_> = vec![Policy1, Policy2].into();
        assert!(PolicyCond::any(vec![Policy1]).satisfy(&check_ps));
        assert!(PolicyCond::any(vec![Policy2]).satisfy(&check_ps));
        assert!(PolicyCond::any(vec![Policy1, Policy2]).satisfy(&check_ps));
        assert!(PolicyCond::any(vec![Policy1, Policy2, Policy3]).satisfy(&check_ps));
    }

    #[test]
    fn any_should_not_satisfy() {
        assert!(!PolicyCond::<TestPolicy>::any(vec![]).satisfy(&vec![].into()));
        assert!(!PolicyCond::any(vec![]).satisfy(&vec![Policy1].into()));
        assert!(!PolicyCond::any(vec![]).satisfy(&vec![Policy1, Policy2].into()));

        // should not satisfy by empty set
        assert!(!PolicyCond::any(vec![Policy1]).satisfy(&vec![].into()));
        assert!(!PolicyCond::any(vec![Policy1, Policy2]).satisfy(&vec![].into()));

        assert!(!PolicyCond::any(vec![Policy1]).satisfy(&vec![Policy2].into()));
        assert!(!PolicyCond::any(vec![Policy1]).satisfy(&vec![Policy2, Policy3].into()));
    }

    #[test]
    fn all_should_satisfy() {
        assert!(PolicyCond::all(vec![Policy1]).satisfy(&vec![Policy1].into()));
        assert!(PolicyCond::all(vec![Policy1, Policy2]).satisfy(&vec![Policy1, Policy2].into()));
        assert!(PolicyCond::all(vec![Policy1, Policy2])
            .satisfy(&vec![Policy1, Policy2, Policy3].into()));
    }

    #[test]
    fn all_should_not_satisfy() {
        assert!(!PolicyCond::<TestPolicy>::all(vec![]).satisfy(&vec![].into()));
        assert!(!PolicyCond::all(vec![]).satisfy(&vec![Policy1].into()));
        assert!(!PolicyCond::all(vec![]).satisfy(&vec![Policy1, Policy2].into()));

        // should not satisfy by empty set
        assert!(!PolicyCond::all(vec![Policy1]).satisfy(&vec![].into()));
        assert!(!PolicyCond::all(vec![Policy1, Policy2]).satisfy(&vec![].into()));

        assert!(!PolicyCond::all(vec![Policy1, Policy2]).satisfy(&vec![Policy1].into()));
        assert!(!PolicyCond::all(vec![Policy1, Policy2]).satisfy(&vec![Policy2].into()));
        assert!(!PolicyCond::all(vec![Policy1, Policy2]).satisfy(&vec![Policy2, Policy3].into()));
    }
}
