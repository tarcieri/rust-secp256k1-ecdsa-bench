//! `k256` ECDSA benchmarks
//! <https://github.com/RustCrypto/elliptic-curves/tree/master/k256>

use criterion::{criterion_group, criterion_main, Criterion};
use k256::ecdsa::{
    signature::{DigestSigner, DigestVerifier},
    Signature, SigningKey,
};
use sha2::{Digest, Sha256};

const EXAMPLE_KEY: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
const EXAMPLE_MSG: &[u8] = b"The Magic Words are Squeamish Ossifrage";

fn sign_ecdsa(c: &mut Criterion) {
    let signing_key = SigningKey::new(&EXAMPLE_KEY).unwrap();
    let msg_digest = Sha256::new().chain(EXAMPLE_MSG);

    c.bench_function("k256 ECDSA signer", move |b| {
        b.iter(|| {
            let _: Signature = signing_key.sign_digest(msg_digest.clone());
        })
    });
}

fn verify_ecdsa(c: &mut Criterion) {
    let signing_key = SigningKey::new(&EXAMPLE_KEY).unwrap();
    let verify_key = signing_key.verify_key();
    let msg_digest = Sha256::new().chain(EXAMPLE_MSG);
    let signature: Signature = signing_key.sign_digest(msg_digest.clone());

    c.bench_function("k256 ECDSA verifier", move |b| {
        b.iter(|| {
            verify_key
                .verify_digest(msg_digest.clone(), &signature)
                .unwrap();
        })
    });
}

criterion_group! {
    name = ecdsa;
    config = Criterion::default();
    targets = sign_ecdsa, verify_ecdsa
}

criterion_main!(ecdsa);
