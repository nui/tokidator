use num_traits::{FromPrimitive, ToPrimitive};

/// A marker trait for enums where variants do not have payloads
///
/// ToPrimitive must produce unique value (same value that use in Ord)
pub trait Permission: Copy + Clone + Ord + FromPrimitive + ToPrimitive {}

pub trait Role: Ord + FromPrimitive {
    type Permission: Permission;

    fn permissions(&self) -> &[Self::Permission];
}
