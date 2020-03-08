use std::collections::HashMap;

#[allow(unused_imports)]
use num_derive::{FromPrimitive, ToPrimitive};
#[allow(unused_imports)]
use strum_macros::{Display, EnumCount};

use lazy_static::lazy_static;

use crate::rbac::policy_set::PolicySet;
use crate::rbac::traits::Role;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Display, FromPrimitive, ToPrimitive, EnumCount)]
pub enum TestPolicy {
    Policy0,
    Policy1,
    Policy2,
    Policy3,
    Policy4,
    Policy5,
    Policy6,
    Policy7,
    Policy8,
    Policy9,
    Policy10,
    Policy11,
    Policy12,
    Policy13,
    Policy14,
    Policy15,
}

impl crate::rbac::Policy for TestPolicy {}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Display, FromPrimitive)]
pub enum TestRole {
    Role0,
    Role1,
    Role2,
}

impl Role for TestRole {
    type Policy = TestPolicy;

    fn as_policy_set_ref(&self) -> Option<&PolicySet<Self::Policy>> {
        POLICIES.get(self)
    }
}

type RolePoliciesMap = HashMap<TestRole, PolicySet<TestPolicy>>;

lazy_static! {
    static ref POLICIES: RolePoliciesMap = create_role_policies();
}

#[inline]
fn create_role_policies() -> RolePoliciesMap {
    use TestPolicy::*;
    use TestRole::*;
    vec![
        (Role0, vec![Policy0, Policy1].into()),
        (Role2, vec![Policy3, Policy4].into()),
    ].into_iter().collect()
}