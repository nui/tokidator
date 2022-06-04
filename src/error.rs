use std::fmt::{self, Debug, Display};

#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Error {
    ExpiredAccessToken,
    InvalidAccessToken,
    InvalidSignedMessage,
    SignatureVerificationFail,
    Unauthorized,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match *self {
            ExpiredAccessToken => f.write_str("expired access token"),
            InvalidAccessToken => f.write_str("invalid access token"),
            InvalidSignedMessage => f.write_str("invalid signed message"),
            SignatureVerificationFail => f.write_str("signature verification fail"),
            Unauthorized => f.write_str("unauthorized"),
        }
    }
}

impl std::error::Error for Error {}
