use crate::crypto::{PrivateKey, PublicKey};

const SEPARATOR: char = '.';

pub struct SignedMessage {
    message: Vec<u8>,
    signature: Vec<u8>,
}

impl SignedMessage {
    pub fn encode(&self) -> String {
        fn base64_encode_buf(input: &[u8], buf: &mut String) {
            base64::encode_config_buf(input, base64::URL_SAFE_NO_PAD, buf)
        }
        let Self { message, signature } = self;
        let mut output = String::with_capacity(self.get_encoded_len());
        base64_encode_buf(message, &mut output);
        output.push(SEPARATOR);
        base64_encode_buf(signature, &mut output);
        output
    }

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

    pub fn decode(s: &str) -> Option<Self> {
        let mut iter = s.split(SEPARATOR);
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
        SEPARATOR.len_utf8()
            + url_safe_no_pad_len(&self.message)
            + url_safe_no_pad_len(&self.signature)
    }
}

fn url_safe_no_pad_len(input: &[u8]) -> usize {
    let x = input.len() * 4;
    (x / 3) + (((x % 3) > 0) as usize)
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
}
