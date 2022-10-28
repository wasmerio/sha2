use std::sync::Mutex;
use wit_bindgen_rust::Handle;

use ::sha2::{Digest, Sha256, Sha512};

wit_bindgen_rust::export!("sha2.wit");

pub struct Hasher(Mutex<Sha256>);

impl sha2::Hasher for Hasher {
    fn sha256() -> Handle<Hasher> {
        Handle::new(Hasher(Mutex::new(Sha256::new())))
    }
    fn update(&self, bytes: Vec<u8>) {
        let mut hasher = self.0.lock().expect("The Mutex was poisoned");
        hasher.update(bytes);
    }
    fn finalize(&self) -> Vec<u8> {
        let hasher = self.0.lock().expect("The Mutex was poisoned");
        hasher.clone().finalize().to_vec()
    }
}
struct Sha2;

impl sha2::Sha2 for Sha2 {
    fn sha256(bytes: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(bytes);
        hasher.finalize().to_vec()
    }
    fn sha512(bytes: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(bytes);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sha2::Sha2 as _;
    use hex_literal::hex;

    #[test]
    fn sha256_string_as_bytes() {
        let sample_string = "hello world";
        let result = Sha2::sha256(sample_string.as_bytes().to_vec());

        let mut hasher = Sha256::new();

        assert_eq!(
            result,
            hex!(
                "
            b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9
        "
            ),
            "The SHA256 hash did not match the sample string {:?}",
            sample_string
        );
    }

    #[test]
    fn sha512_string_as_bytes() {
        let sample_string = "hello world";
        let result = Sha2::sha512(sample_string.as_bytes().to_vec());
        assert_eq!(
            result,
            hex!(
                "
                309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f
                989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f
        "
            ),
            "The SHA512 hash did not match the sample string {:?}",
            sample_string
        );
    }
}
