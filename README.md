# Rust secp256k1 ECDSA benchmarks

This repository contains ECDSA benchmarks for three different Rust crates that
implement the secp256k1 elliptic curve:

- [`k256`]: RustCrypto's pure Rust implementation of secp256k1
- [`libsecp256k1`]: Parity's implementation of secp256k1
- [`rust-secp256k1`]: FFI wrapper crate for the [`bitcoin-core/secp256k1`]
  C library

[`k256`]: https://github.com/RustCrypto/elliptic-curves/tree/master/k256
[`libsecp256k1`]: https://github.com/paritytech/libsecp256k1
[`rust-secp256k1`]: https://github.com/rust-bitcoin/rust-secp256k1
[`bitcoin-core/secp256k1`]: https://github.com/bitcoin-core/secp256k1

## Initial results

Benchmarks performed on a Intel(R) Xeon(R) E-2286M  CPU @ 2.40GHz with
Turbo Boost disabled.

Endomorphism optimizations were enabled for both the `k256` and
`rust-secp256k1` crates.

![bencmark results](https://raw.githubusercontent.com/tarcieri/rust-secp256k1-ecdsa-bench/main/img/rust-secp256k1-ecdsa.png)

## License

Licensed under either of the following, at your option:

- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)
