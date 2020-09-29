//! `libsecp256k1` ECDSA benchmarks
//! <https://github.com/paritytech/libsecp256k1>

use criterion::{criterion_group, criterion_main, Criterion};
use libsecp256k1::{Message, PublicKey, SecretKey};
use sha2::{Digest, Sha256};

const EXAMPLE_KEY: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
const EXAMPLE_MSG: &[u8] = b"The Magic Words are Squeamish Ossifrage";

fn sign_ecdsa(c: &mut Criterion) {
    let secret_key = SecretKey::parse(&EXAMPLE_KEY).unwrap();
    let msg = Message::parse(Sha256::digest(EXAMPLE_MSG).as_ref());

    c.bench_function("libsecp256k1 ECDSA signer", move |b| {
        b.iter(|| {
            libsecp256k1::sign(&msg, &secret_key);
        })
    });
}

fn verify_ecdsa(c: &mut Criterion) {
    let secret_key = SecretKey::parse(&EXAMPLE_KEY).unwrap();
    let public_key = PublicKey::from_secret_key(&secret_key);
    let msg = Message::parse(Sha256::digest(EXAMPLE_MSG).as_ref());
    let signature = libsecp256k1::sign(&msg, &secret_key).0;

    c.bench_function("libsecp256k1 ECDSA verifier", move |b| {
        b.iter(|| {
            libsecp256k1::verify(&msg, &signature, &public_key);
        })
    });
}

criterion_group! {
    name = ecdsa;
    config = Criterion::default();
    targets = sign_ecdsa, verify_ecdsa
}

criterion_main!(ecdsa);
