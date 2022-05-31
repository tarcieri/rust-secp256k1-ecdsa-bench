//! `rust-secp256k1` ECDSA benchmarks
//! <https://github.com/rust-bitcoin/rust-secp256k1>

use criterion::{criterion_group, criterion_main, Criterion};
use rust_secp256k1::{Message, PublicKey, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

const EXAMPLE_KEY: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
const EXAMPLE_MSG: &[u8] = b"The Magic Words are Squeamish Ossifrage";

fn sign_ecdsa(c: &mut Criterion) {
    let engine = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&EXAMPLE_KEY).unwrap();
    let msg = Message::from_slice(Sha256::digest(EXAMPLE_MSG).as_ref()).unwrap();

    c.bench_function("rust-secp256k1 ECDSA signer", move |b| {
        b.iter(|| engine.sign_ecdsa(&msg, &secret_key))
    });
}

fn verify_ecdsa(c: &mut Criterion) {
    let engine = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&EXAMPLE_KEY).unwrap();
    let public_key = PublicKey::from_secret_key(&engine, &secret_key);
    let msg = Message::from_slice(Sha256::digest(EXAMPLE_MSG).as_ref()).unwrap();
    let signature = engine.sign_ecdsa(&msg, &secret_key);

    c.bench_function("rust-secp256k1 ECDSA verifier", move |b| {
        b.iter(|| {
            engine.verify_ecdsa(&msg, &signature, &public_key).unwrap();
        })
    });
}

criterion_group! {
    name = ecdsa;
    config = Criterion::default();
    targets = sign_ecdsa, verify_ecdsa
}

criterion_main!(ecdsa);
