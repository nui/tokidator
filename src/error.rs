#[derive(Debug, Clone)]
pub enum Error {
    SignatureVerificationFail,
    BadPolicyEncoding,
    BadSignedMessageEncoding,
    Forbidden,
    ExpiredAccessToken,
    Unauthorized,
}