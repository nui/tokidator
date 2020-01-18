use std::str::FromStr;

use crate::crypto::{PrivateKey, PublicKey};

const SEPARATOR: &str = ".";

pub struct SignedMessage {
    message: Vec<u8>,
    signature: Vec<u8>,
}

impl SignedMessage {
    pub fn to_string(&self) -> String {
        fn encode(input: &[u8]) -> String {
            base64::encode_config(input, base64::URL_SAFE_NO_PAD)
        }
        [encode(&self.message), encode(&self.signature)].join(SEPARATOR)
    }

    pub fn create(message: Vec<u8>, key: &PrivateKey) -> Self {
        let signature = key.sign(&message);
        Self {
            message,
            signature,
        }
    }

    pub fn verify(&self, key: &PublicKey) -> bool {
        key.verify(&self.message, &self.signature)
    }

    pub fn message(&self) -> &[u8] {
        &self.message
    }

    pub fn signature(&self) -> &[u8] {
        &self.signature
    }
}

impl FromStr for SignedMessage {
    type Err = super::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(SEPARATOR);
        let decode = |input| {
            base64::decode_config(input, base64::URL_SAFE_NO_PAD)
                .map_err(|_| Self::Err::BadSignedMessageEncoding)
        };
        match (iter.next(), iter.next()) {
            (Some(message), Some(signature)) => {
                Ok(SignedMessage {
                    message: decode(message)?,
                    signature: decode(signature)?,
                })
            }
            _ => Err(Self::Err::BadSignedMessageEncoding)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto::tests::{get_test_private_key, get_test_public_key};

    use super::*;

    #[test]
    fn serialization() {
        let sm1 = SignedMessage {
            message: "message".as_bytes().to_vec(),
            signature: "signature".as_bytes().to_vec(),
        };
        let sm2 = SignedMessage::from_str(&sm1.to_string()).unwrap();
        assert_eq!(sm1.message, sm2.message);
        assert_eq!(sm1.signature, sm2.signature);
    }

    #[test]
    fn create_should_return_predicable_result() {
        let message = "message".as_bytes().to_vec();
        let key = PrivateKey::from_base64_encoded(&get_test_private_key()).unwrap();
        let sm = SignedMessage::create(message, &key);
        assert_eq!(sm.to_string(), String::from("bWVzc2FnZQ.gH3fe9YO9tEv7f8adiZ2w7F6-7doNp3yyaDrfWuQNCuJi6bwF2jqm7v4p-wANdOahO1wvULOH96JJDnQlUoEDw"));
    }

    #[test]
    fn should_verify_previous_encoded() {
        let sm = SignedMessage::from_str("bWVzc2FnZQ.gH3fe9YO9tEv7f8adiZ2w7F6-7doNp3yyaDrfWuQNCuJi6bwF2jqm7v4p-wANdOahO1wvULOH96JJDnQlUoEDw")
            .expect("valid encoded");
        let public_key = PublicKey::from_base64_encoded(&get_test_public_key()).unwrap();
        assert!(sm.verify(&public_key));
    }
}