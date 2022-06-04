use std::collections::BTreeMap;

use once_cell::sync::Lazy;

use crate::rbac::traits::Role;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    num_derive::FromPrimitive,
    num_derive::ToPrimitive,
    strum::Display,
    strum::EnumCount,
)]
#[repr(u16)]
pub enum TestPermission {
    Permission0,
    Permission1,
    Permission2,
    Permission3,
    Permission4,
    Permission5,
    Permission6,
    Permission7,
    Permission8,
    Permission9,
    Permission10,
    Permission11,
    Permission12,
    Permission13,
    Permission14,
    Permission15,
}

impl crate::rbac::Permission for TestPermission {}

#[derive(
    Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, num_derive::FromPrimitive, strum::Display,
)]
pub enum TestRole {
    Role0,
    Role1,
    Role2,
}

impl Role for TestRole {
    type Permission = TestPermission;

    fn permissions(&self) -> &[Self::Permission] {
        PERMISSIONS.get(self).map(Vec::as_slice).unwrap_or_default()
    }
}

type RolePermissionsMap = BTreeMap<TestRole, Vec<TestPermission>>;

static PERMISSIONS: Lazy<RolePermissionsMap> = Lazy::new(create_role_permissions);

fn create_role_permissions() -> RolePermissionsMap {
    use TestPermission::*;
    use TestRole::*;
    let mut map = RolePermissionsMap::new();
    map.insert(Role0, vec![Permission0, Permission1]);
    map.insert(Role2, vec![Permission3, Permission4]);
    map
}
