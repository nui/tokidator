use std::hash::Hash;

use num_traits::{FromPrimitive, ToPrimitive};

use super::PolicySet;

pub trait Policy: Copy + Clone + Hash + Eq + FromPrimitive + ToPrimitive {}

pub trait Role: Hash + Eq + FromPrimitive {
    type Policy: Policy;

    fn as_policy_set_ref(&self) -> Option<&PolicySet<Self::Policy>> {
        None
    }
}