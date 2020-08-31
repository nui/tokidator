use num_traits::FromPrimitive;

pub use policy_cond::PolicyCond;
pub use policy_set::PolicySet;
pub use role_set::RoleSet;
pub use traits::{Policy, Role};

#[cfg(test)]
pub mod test_helpers;

mod policy_cond;
mod policy_set;
mod role_set;
mod traits;

#[inline]
pub fn json_discriminant_array_to_vec<T: FromPrimitive>(
    unparsed: &str,
) -> Result<Vec<T>, (usize, Vec<T>)> {
    unparsed
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .filter_map(|s: &str| s.trim().parse::<usize>().ok())
        .try_fold(Vec::new(), |mut policies, id| {
            if let Some(p) = T::from_usize(id) {
                policies.push(p);
                Ok(policies)
            } else {
                Err((id, policies))
            }
        })
}

#[cfg(test)]
mod tests {
    use test_helpers::TestPolicy::{self, *};

    use super::*;

    #[test]
    fn test_json_discriminant_array_to_vec() {
        let unparsed = "[0, 2, 5, 8]";
        let actual = json_discriminant_array_to_vec::<TestPolicy>(unparsed);
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), vec![Policy0, Policy2, Policy5, Policy8]);

        let unparsed = "[1, 3, 999]";
        let actual = json_discriminant_array_to_vec::<TestPolicy>(unparsed);
        assert!(actual.is_err());
        let (id, v) = actual.unwrap_err();
        assert_eq!(id, 999);
        assert_eq!(v, vec![Policy1, Policy3]);
    }
}
