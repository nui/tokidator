use std::num::NonZeroU32;

use rand::RngCore;
use rand::rngs::OsRng;
use ring::pbkdf2::{derive, PBKDF2_HMAC_SHA256};
use ring::signature::{Ed25519KeyPair, KeyPair};

fn main() {
    let mut secret = [0u8; 32];
    let mut salt = [0u8; 32];

    let mut key = [0u8; 32];

    let mut count = 0;
    OsRng.fill_bytes(&mut secret);
    OsRng.fill_bytes(&mut salt);
    loop {
        derive(PBKDF2_HMAC_SHA256, NonZeroU32::new(100_000 + count).unwrap(), &salt, &secret, &mut key);
        let key_pair = Ed25519KeyPair::from_seed_unchecked(&key).unwrap();
        let private_key = base64::encode_config(&key, base64::URL_SAFE_NO_PAD);
        let public_key = base64::encode_config(key_pair.public_key().as_ref(), base64::URL_SAFE_NO_PAD);
        count += 1;
        // skip key with - or _ for ease of coping
        if private_key.contains("-") || private_key.contains("_") || public_key.contains("_") || public_key.contains("-") {
            continue;
        }
        println!("Public part: {}\n", &public_key);
        println!("SECRET part: {} <- keep this secure\n", &private_key);
//        println!("\nDebugging Info: skip {} keys\n", count - 1);
        break;
    }
}