use protobuf::{parse_from_bytes, Message};

use crate::rbac::test_helpers::TestPolicy;
use crate::rbac::PolicySet;
use crate::token::PolicyAccessToken;

#[derive(Debug)]
pub struct TestAccessToken {
    policies: PolicySet<TestPolicy>,
    expired: bool,
}

impl TestAccessToken {
    pub fn new(policies: PolicySet<TestPolicy>, expired: bool) -> TestAccessToken {
        Self { policies, expired }
    }
}

impl PolicyAccessToken for TestAccessToken {
    type Policy = TestPolicy;

    fn policies(&self) -> &PolicySet<Self::Policy> {
        &self.policies
    }

    fn is_expired(&self) -> bool {
        self.expired
    }

    fn to_bytes(&self) -> Vec<u8> {
        let policies = self.policies.to_bytes();
        let mut builder = crate::protos::TestAccessToken::new();
        builder.set_policies(policies);
        builder.set_expired(self.expired);
        builder
            .write_to_bytes()
            .expect("Fail build bytes from test policy")
    }

    fn from_bytes(buf: &[u8]) -> Option<Self> {
        let token = parse_from_bytes::<crate::protos::TestAccessToken>(buf).ok()?;
        let ps = PolicySet::parse_from_bytes(token.policies.as_slice())
            .expect("Bad encoded test policies");
        Some(Self::new(ps, token.expired))
    }
}
