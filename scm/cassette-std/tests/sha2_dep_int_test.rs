//! Dependency coverage test for `sha2`.
//! @covers: sha2

use sha2::{Digest, Sha256};

/// @covers: sha2
/// Confirms SHA256 produces the known digest for "abc" — the same
/// computation used by `CassetteLayer::sha256_hex` for body fingerprinting.
#[test]
fn std_struct_sha2_dep_known_vector_int_test() {
    let mut h = Sha256::new();
    h.update(b"abc");
    let result = hex::encode(h.finalize());
    assert_eq!(
        result, "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        "SHA256('abc') must match NIST test vector"
    );
}

/// @covers: sha2
/// Confirms SHA256 output is always 32 bytes regardless of input size.
#[test]
fn std_struct_sha2_dep_output_always_32_bytes_int_test() {
    let mut h = Sha256::new();
    h.update(b"");
    let result = h.finalize();
    assert_eq!(result.len(), 32, "SHA256 output must always be 32 bytes");
}
