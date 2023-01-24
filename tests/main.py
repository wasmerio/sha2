from sha2 import bindings
from sha2.bindings.sha2 import Hasher

sha2 = bindings.sha2()


def test_sha256_compress():
    assert (
        sha2.sha256(b"hello world").hex()
        == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    )


def test_sha512_compress():
    assert (
        sha2.sha512(b"hello world").hex()
        == "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
    )


def test_hasher_sha256():
    hasher = Hasher.sha256(sha2)
    hasher.update(b"hello")
    hasher.update(b" ")
    hasher.update(b"world")

    # test against sample hash256
    assert (
        hasher.finalize().hex()
        == "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
    )

    # drop the hasher
    hasher.drop()


def test_hasher_sha512():
    hasher = Hasher.sha512(sha2)
    hasher.update(b"hello")
    hasher.update(b" ")
    hasher.update(b"world")

    # test against sample hash512
    assert (
        hasher.finalize().hex()
        == "309ecc489c12d6eb4cc40f50c902f2b4d0ed77ee511a7c7a9bcd3ca86d4cd86f989dd35bc5ff499670da34255b45b0cfd830e81f605dcf7dc5542e93ae9cd76f"
    )

    # drop the hasher
    hasher.drop()
