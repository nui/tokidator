#[derive(Debug, Clone)]
pub enum Error {
    SignatureVerificationFail,
    BadAccessTokenEncoding,
    BadSignedMessageEncoding,
    Forbidden,
    ExpiredAccessToken,
    Unauthorized,
}
