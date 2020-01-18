pub use condition::PolicyCondition;
pub use set::PolicySet;

#[cfg(test)]
pub mod tests;

mod set;
mod condition;

pub trait PolicyCount {
    fn count() -> usize;
}

#[cfg(feature = "strum")]
impl<P> PolicyCount for P where
    P: strum::EnumCount {
    fn count() -> usize {
        <P as strum::EnumCount>::count()
    }
}