use protobuf::Message;

use crate::rbac::test_helpers::TestPermission;
use crate::rbac::PermissionSet;

use super::AccessToken;

#[derive(Debug)]
pub struct TestAccessToken {
    permissions: PermissionSet<TestPermission>,
    expired: bool,
}

impl TestAccessToken {
    pub fn new(permissions: PermissionSet<TestPermission>, expired: bool) -> TestAccessToken {
        Self {
            permissions,
            expired,
        }
    }
}

impl AccessToken for TestAccessToken {
    type Permission = TestPermission;
    type ParseError = ();

    fn from_bytes(buf: &[u8]) -> Result<Self, Self::ParseError> {
        let token = crate::protos::token::TestAccessToken::parse_from_bytes(buf).map_err(drop)?;
        let ps = PermissionSet::parse_from_bytes(token.permissions.as_slice())
            .expect("Bad encoded test permissions");
        Ok(Self::new(ps, token.expired))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let permissions = self.permissions.to_bytes();
        let mut builder = crate::protos::token::TestAccessToken::new();
        builder.permissions = permissions;
        builder.expired = self.expired;
        builder
            .write_to_bytes()
            .expect("Fail build bytes from test permission")
    }

    fn is_expired(&self) -> bool {
        self.expired
    }

    fn permissions(&self) -> &PermissionSet<Self::Permission> {
        &self.permissions
    }
}
