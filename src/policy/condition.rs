use std::hash::Hash;

use num_traits::{FromPrimitive, ToPrimitive};

use crate::policy::{PolicyCount, PolicySet};

pub enum PolicyCondition<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    NoCheck,
    Contains(P),
    OneOf(PolicySet<P>),
    AllOf(PolicySet<P>),
}

impl<P> PolicyCondition<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    pub fn satisfy(&self, policies: &PolicySet<P>) -> bool {
        use PolicyCondition::*;
        match self {
            NoCheck => true,
            Contains(policy) => policies.contains(policy),
            OneOf(set) =>
                set.iter().any(|p| policies.contains(p)),
            AllOf(set) => {
                !set.is_empty() && set.iter().all(|p| policies.contains(p))
            }
        }
    }
}

impl<P> From<P> for PolicyCondition<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn from(p: P) -> Self {
        PolicyCondition::Contains(p)
    }
}

impl<P> From<Vec<P>> for PolicyCondition<P>
    where P: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount {
    fn from(vec: Vec<P>) -> Self {
        PolicyCondition::AllOf(vec.into_iter().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::PolicyCondition::*;
    use crate::policy::tests::TestPolicy::{self, *};

    use super::*;

    #[test]
    fn no_check_should_satisfy() {
        assert!(NoCheck.satisfy(&PolicySet::<TestPolicy>::new()));
        assert!(NoCheck.satisfy(&vec![Policy1].into()));
        assert!(NoCheck.satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_satisfy() {
        assert!(Contains(Policy1).satisfy(&vec![Policy1].into()));
        assert!(Contains(Policy1).satisfy(&vec![Policy1, Policy2].into()));
    }

    #[test]
    fn contains_should_not_satisfy() {
        assert!(!Contains(Policy1).satisfy(&vec![].into()));
        assert!(!Contains(Policy1).satisfy(&vec![Policy2].into()));
    }

    #[test]
    fn one_of_should_satisfy() {
        let check_ps: PolicySet<_> = vec![Policy1, Policy2].into();
        assert!(OneOf(vec![Policy1].into()).satisfy(&check_ps));
        assert!(OneOf(vec![Policy2].into()).satisfy(&check_ps));
        assert!(OneOf(vec![Policy1, Policy2].into()).satisfy(&check_ps));
        assert!(OneOf(vec![Policy1, Policy2, Policy3].into()).satisfy(&check_ps));
    }

    #[test]
    fn one_of_should_not_satisfy() {
        assert!(!OneOf(Vec::<TestPolicy>::new().into()).satisfy(&vec![].into()));
        assert!(!OneOf(vec![].into()).satisfy(&vec![Policy1].into()));
        assert!(!OneOf(vec![].into()).satisfy(&vec![Policy1, Policy2].into()));

        // should not satisfy by empty set
        assert!(!OneOf(vec![Policy1].into()).satisfy(&vec![].into()));
        assert!(!OneOf(vec![Policy1, Policy2].into()).satisfy(&vec![].into()));

        assert!(!OneOf(vec![Policy1].into()).satisfy(&vec![Policy2].into()));
        assert!(!OneOf(vec![Policy1].into()).satisfy(&vec![Policy2, Policy3].into()));
    }

    #[test]
    fn all_of_should_satisfy() {
        assert!(AllOf(vec![Policy1].into()).satisfy(&vec![Policy1].into()));
        assert!(AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![Policy1, Policy2].into()));
        assert!(AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![Policy1, Policy2, Policy3].into()));
    }

    #[test]
    fn all_of_should_not_satisfy() {
        assert!(!AllOf(Vec::<TestPolicy>::new().into()).satisfy(&vec![].into()));
        assert!(!AllOf(vec![].into()).satisfy(&vec![Policy1].into()));
        assert!(!AllOf(vec![].into()).satisfy(&vec![Policy1, Policy2].into()));

        // should not satisfy by empty set
        assert!(!AllOf(vec![Policy1].into()).satisfy(&vec![].into()));
        assert!(!AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![].into()));

        assert!(!AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![Policy1].into()));
        assert!(!AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![Policy2].into()));
        assert!(!AllOf(vec![Policy1, Policy2].into()).satisfy(&vec![Policy2, Policy3].into()));
    }
}