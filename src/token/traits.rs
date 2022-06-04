use crate::rbac::{Permission, PermissionSet, Predicate};

pub trait AccessToken: Sized {
    type Permission: Permission;
    type ParseError;

    fn from_bytes(buf: &[u8]) -> Result<Self, Self::ParseError>;
    fn to_bytes(&self) -> Vec<u8>;

    fn is_expired(&self) -> bool;
    fn permissions(&self) -> &PermissionSet<Self::Permission>;

    fn is_authorized<P>(&self, predicate: P) -> bool
    where
        P: AsRef<Predicate<Self::Permission>>,
    {
        predicate.as_ref().satisfy(self.permissions())
    }
}
