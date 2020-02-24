use std::ops::Deref;

pub use validator::AccessEnforcer;
pub use validator::ValidationAuthority;

use crate::rbac::{Policy, PolicySet};

pub trait PolicyAccessToken: Sized {
    type Policy: Policy;

    fn policies(&self) -> &PolicySet<Self::Policy>;
    fn is_expired(&self) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Option<Self>;
}

pub trait ToTokenStr {
    fn to_token_str(&self) -> Option<&str>;
}

impl<T: Deref<Target=str>> ToTokenStr for Option<T> {
    fn to_token_str(&self) -> Option<&str> {
        self.as_deref()
    }
}

#[cfg(test)]
pub mod test_utils;

mod validator;
