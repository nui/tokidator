use std::hash::Hash;
use std::ops::Deref;

use num_traits::{FromPrimitive, ToPrimitive};

pub use validator::AccessEnforcer;
pub use validator::ValidationAuthority;

use crate::policy::{PolicyCount, PolicySet};

pub trait PolicyAccessToken: Sized {
    type Policy: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount;

    fn policies(&self) -> &PolicySet<Self::Policy>;
    fn is_expired(&self) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Option<Self>;
}

pub trait ToTokenStr {
    fn to_str(&self) -> Option<&str>;
}

impl<T> ToTokenStr for Option<T>
    where T: Deref<Target=str> {
    fn to_str(&self) -> Option<&str> {
        self.as_deref()
    }
}

#[cfg(test)]
pub mod tests;

mod validator;
