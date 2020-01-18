use std::hash::Hash;

use num_traits::{FromPrimitive, ToPrimitive};

pub use validator::AccessTokenValidator;
pub use validator::AccessValidator;

use crate::policy::{PolicyCount, PolicySet};

pub trait PolicyAccessToken: Sized {
    type Policy: Hash + Eq + FromPrimitive + ToPrimitive + PolicyCount;

    fn policies(&self) -> &PolicySet<Self::Policy>;
    fn is_expired(&self) -> bool;
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(buf: &[u8]) -> Option<Self>;
}

pub trait TokenExtractor {
    fn extract_access_token(&self) -> Option<&str>;
}

impl TokenExtractor for Option<&str> {
    fn extract_access_token(&self) -> Option<&str> {
        *self
    }
}

#[cfg(test)]
pub mod tests;

mod validator;
