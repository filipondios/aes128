# AES128

Rust implementation of the Advanced Encryption Standard (AES).

> [!WARNING]  
> This implementation is just made for fun so it should not be used in production.
> Its limited to operating with 128-bit blocks of input and corresponding 
> encrypted 128-bit blocks of output instead of streams of bytes. This implementation
> also does not have a cryptographically secure key generation algorithm.

## Usage

The usage of this algorithm is dead simple, there are only two possible available 
functions for the user: `aes::encrypt_block` and `aes::decrypt_block`. You have
already an example at `src/main.rs` similar to this code below:

```rust
let block: [u8; 16] = [
  0x54, 0x68, 0x61, 0x74,
  0x73, 0x20, 0x6D, 0x79,
  0x20, 0x4B, 0x75, 0x6E,
  0x67, 0x20, 0x46, 0x75,
];

let key: [u8; 16] = [
  0x54, 0x77, 0x6F, 0x20,
  0x4F, 0x6E, 0x65, 0x20,
  0x4E, 0x69, 0x6E, 0x65,
  0x20, 0x54, 0x77, 0x6F,
];

let cipher = aes::encrypt_block(block,  key);
let plain  = aes::decrypt_block(cipher, key);
```

Oviously both functions expect two 128-bit blocks (represented as two 8-bit
unsigned integer arrays), one for the block to be encrypted or decrypted and the key.
The output to the previous code must be the following:

## References

- [AES by NIST, NIST FIPS 197-upd1](https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf)
- [AES Example](https://www.kavaliro.com/wp-content/uploads/2014/03/AES.pdf)
