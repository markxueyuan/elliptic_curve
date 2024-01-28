In this package we build the elliptic curves on finite fields.

All primitive integer types as well as [Cryto-bigint](https://docs.rs/crypto-bigint/0.5.5/crypto_bigint/index.html#) types are supported in instantiating the points and the parameters of the elliptic curves.

As a special case of the elliptic curves, the secp256k1 specification used in Bitcoin's public-key generation is implemented. One can use the [secp256k1-key-gen](/bin/secp256k1_key_gen.rs) program to generate the secp256k1 keypairs in command lines.

This project needs to be used in combination with the [finite_field](https://github.com/markxueyuan/finite_field) package.