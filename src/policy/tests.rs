#[allow(unused_imports)]
use num_derive::{FromPrimitive, ToPrimitive};
#[allow(unused_imports)]
use strum_macros::{Display, EnumCount};

#[cfg(test)]
#[derive(Debug, Hash, Eq, PartialEq, Display, FromPrimitive, ToPrimitive, EnumCount)]
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