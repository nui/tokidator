use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, ED25519};

pub struct PrivateKey(Ed25519KeyPair);

impl PrivateKey {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        Ed25519KeyPair::from_seed_unchecked(bytes).map(Self).ok()
    }

    pub fn sign(&self, msg: &[u8]) -> Vec<u8> {
        self.0.sign(msg).as_ref().to_vec()
    }

    pub fn from_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> Option<Self> {
        base64::decode_config(input, base64::URL_SAFE_NO_PAD)
            .ok()
            .and_then(|seed| Self::from_bytes(&seed))
    }
}

#[derive(Clone)]
pub struct PublicKey(UnparsedPublicKey<Vec<u8>>);

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(UnparsedPublicKey::new(&ED25519, bytes.to_vec()))
    }

    pub fn from_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> Option<Self> {
        base64::decode_config(input, base64::URL_SAFE_NO_PAD)
            .map(|bytes| Self(UnparsedPublicKey::new(&ED25519, bytes)))
            .ok()
    }

    pub fn verify(&self, message: &[u8], signature: &[u8]) -> bool {
        self.0.verify(message, signature).is_ok()
    }
}

const SEPARATOR: u8 = b'.';

pub struct SignedMessage {
    message: Vec<u8>,
    signature: Vec<u8>,
}

impl SignedMessage {
    pub fn create(message: Vec<u8>, key: &PrivateKey) -> Self {
        let signature = key.sign(&message);
        Self { message, signature }
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

    pub fn encode(&self) -> String {
        fn base64_encode_buf(input: &[u8], buf: &mut String) {
            base64::encode_config_buf(input, base64::URL_SAFE_NO_PAD, buf)
        }
        let Self { message, signature } = self;
        let mut output = String::with_capacity(self.get_encoded_len());
        base64_encode_buf(message, &mut output);
        output.push(char::from(SEPARATOR));
        base64_encode_buf(signature, &mut output);
        output
    }

    pub fn decode<T: AsRef<[u8]>>(input: T) -> Option<Self> {
        let mut iter = input.as_ref().split(|&b| b == SEPARATOR);
        let decode = |input| base64::decode_config(input, base64::URL_SAFE_NO_PAD).ok();
        match (iter.next(), iter.next()) {
            (Some(message), Some(signature)) => Some(SignedMessage {
                message: decode(message)?,
                signature: decode(signature)?,
            }),
            _ => None,
        }
    }

    fn get_encoded_len(&self) -> usize {
        char::from(SEPARATOR).len_utf8()
            + url_safe_no_pad_len(&self.message)
            + url_safe_no_pad_len(&self.signature)
    }
}

/// Calculate perfect base64 encoded size
///
/// Each base64 character can store 6 bits. One byte use 8 bits.
/// Total number of required bytes is `n * 8 / 6`
///
/// note: `n * 8 / 6` is equal to `n * 4 / 3`.
const fn url_safe_no_pad_len(input: &[u8]) -> usize {
    // TODO: Refactor to `(input.len() * 4).div_ceil(3)` when it got stabilized
    div_ceil(input.len() * 4, 3)
}

/// Copied from std implementation
const fn div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 {
        d + 1
    } else {
        d
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub fn get_test_public_key() -> String {
        String::from("y9OTFvZmHe41kMjCYtDd8574bv46CSDKexUKN9R7mgM")
    }

    pub fn get_test_private_key() -> String {
        String::from("aMWX1G0p36BRx7YqAJaBJ7hnMDxqIbln0toRQcWQfoA")
    }

    #[test]
    fn serialization() {
        let sm1 = SignedMessage {
            message: "message".as_bytes().to_vec(),
            signature: "signature".as_bytes().to_vec(),
        };
        let sm2 = SignedMessage::decode(&sm1.encode()).unwrap();
        assert_eq!(sm1.message, sm2.message);
        assert_eq!(sm1.signature, sm2.signature);
    }

    #[test]
    fn create_should_return_predicable_result() {
        let message = "message".as_bytes().to_vec();
        let key = PrivateKey::from_base64(&get_test_private_key()).unwrap();
        let sm = SignedMessage::create(message, &key);
        assert_eq!(sm.encode(), String::from("bWVzc2FnZQ.gH3fe9YO9tEv7f8adiZ2w7F6-7doNp3yyaDrfWuQNCuJi6bwF2jqm7v4p-wANdOahO1wvULOH96JJDnQlUoEDw"));
    }

    #[test]
    fn should_verify_previous_encoded() {
        let sm = SignedMessage::decode("bWVzc2FnZQ.gH3fe9YO9tEv7f8adiZ2w7F6-7doNp3yyaDrfWuQNCuJi6bwF2jqm7v4p-wANdOahO1wvULOH96JJDnQlUoEDw")
            .expect("valid encoded");
        let public_key = PublicKey::from_base64(&get_test_public_key()).unwrap();
        assert!(sm.verify(&public_key));
    }

    #[test]
    fn verify_url_safe_no_pad_len() {
        let bytes: &[u8] = &[0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(url_safe_no_pad_len(&bytes[..0]), 0);
        assert_eq!(url_safe_no_pad_len(&bytes[..1]), 2);
        assert_eq!(url_safe_no_pad_len(&bytes[..2]), 3);
        assert_eq!(url_safe_no_pad_len(&bytes[..3]), 4);
        assert_eq!(url_safe_no_pad_len(&bytes[..4]), 6);
        assert_eq!(url_safe_no_pad_len(&bytes[..5]), 7);
        assert_eq!(url_safe_no_pad_len(&bytes[..6]), 8);
        assert_eq!(url_safe_no_pad_len(&bytes[..7]), 10);
        assert_eq!(url_safe_no_pad_len(&bytes[..8]), 11);
    }
}
