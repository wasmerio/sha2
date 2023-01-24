use std::sync::Mutex;
use wai_bindgen_rust::Handle;

use original::{Digest, Sha256, Sha512};

wai_bindgen_rust::export!("sha2.wai");

pub trait Crypt {
    fn update_hash(&mut self, bytes: Vec<u8>);
    fn finalize_hash(&self) -> Vec<u8>;
}

impl<T: Digest + Clone> Crypt for T {
    fn update_hash(&mut self, bytes: Vec<u8>) {
        self.update(bytes);
    }

    fn finalize_hash(&self) -> Vec<u8> {
        self.clone().finalize().to_vec()
    }
}

pub struct Hasher(Mutex<Box<dyn Crypt>>);

impl Hasher {
    fn new(state: impl Crypt + 'static) -> Self {
        Hasher(Mutex::new(Box::new(state)))
    }
}

impl sha2::Hasher for Hasher {
    fn sha256() -> Handle<Hasher> {
        Handle::new(Hasher::new(Sha256::default()))
    }
    fn sha512() -> Handle<Hasher> {
        Handle::new(Hasher::new(Sha512::default()))
    }
    fn update(&self, bytes: Vec<u8>) {
        let mut hasher = self.0.lock().expect("The Mutex was poisoned");
        hasher.update_hash(bytes);
    }
    fn finalize(&self) -> Vec<u8> {
        let hasher = self.0.lock().expect("The Mutex was poisoned");
        hasher.finalize_hash()
    }
}
struct Sha2;

impl sha2::Sha2 for Sha2 {
    fn sha256(bytes: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update_hash(bytes);
        hasher.finalize().to_vec()
    }
    fn sha512(bytes: Vec<u8>) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update_hash(bytes);
        hasher.finalize().to_vec()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::sha2::Hasher as _;
    use crate::sha2::Sha2 as _;
    use hex_literal::hex;

    #[test]
    fn sha256_string_as_bytes() {
        let sample_string = "hello world";

        let result = Sha2::sha256(sample_string.as_bytes().to_vec());

        assert_eq!(
            result,
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"),
            "The SHA256 hash did not match the sample string {sample_string:?}",
        );
    }
    #[test]
    fn sha512_string_as_bytes() {
        let sample_string = "hello world";

        let result = Sha2::sha512(sample_string.as_bytes().to_vec());

        assert_eq!(
            result,
            hex!(
                "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f"
                "989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
            ),
            "The SHA512 hash did not match the sample string {sample_string:?}",
        );
    }

    #[test]
    fn sha256_hasher() {
        let hasher = Hasher::new(Sha256::new());
        hasher.update("hello".into());
        hasher.update(" ".into());
        hasher.update("world".into());

        let result = hasher.finalize();

        assert_eq!(
            result,
            hex!("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"),
            "The SHA256 hash did not match the sample string, hello world",
        );
    }

    #[test]
    fn sha512_hasher() {
        let hasher = Hasher::new(Sha512::new());
        hasher.update("hello".into());
        hasher.update(" ".into());
        hasher.update("world".into());

        let result = hasher.finalize();

        assert_eq!(
            result,
            hex!(
                "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f"
                "989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
            ),
            "The SHA512 hash did not match the sample string, hello world",
        );
    }
}
