use num_traits::{FromPrimitive, ToPrimitive};

use super::PolicySet;

pub trait Policy: Copy + Clone + Ord + Eq + FromPrimitive + ToPrimitive {}

pub trait Role: Ord + Eq + FromPrimitive {
    type Policy: Policy;

    fn as_policy_set_ref(&self) -> Option<&PolicySet<Self::Policy>> {
        None
    }
}
