use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
pub enum Error {
    SignatureVerificationFail,
    BadAccessTokenEncoding,
    BadSignedMessageEncoding,
    Forbidden,
    ExpiredAccessToken,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            SignatureVerificationFail => f.write_str("signature verification fail"),
            BadAccessTokenEncoding => f.write_str("bad access token encoding"),
            BadSignedMessageEncoding => f.write_str("bad signed message encoding"),
            Forbidden => f.write_str("forbidden"),
            ExpiredAccessToken => f.write_str("expired access token"),
            Unauthorized => f.write_str("unauthorized"),
        }
    }
}

impl std::error::Error for Error {}
