# AES128

## About

In this repository you will find the implementation of the <a href="https://es.wikipedia.org/wiki/Advanced_Encryption_Standard">AES (Advanced Encryption Standard)</a> block cipher that was originally proposed by <a href="https://www.nist.gov/">NIST</a>. The formal document on which this implementation is based is <a href="https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.197.pdf">FIPS.197.pdf</a>. Please note that this code is for demonstration purposes only and not for other use.

## Considerations

This code is not recommended for use in document or text encryption of relevance, since, among many reasons, it does not offer a padding mechanism and the mode of operation is not as secure as <a href="https://en.wikipedia.org/wiki/Galois/Counter_Mode">AES GCM</a> could be. Moreover, as its name indicates, it is the 128 bit version, which is the weakest of the three possible implementations (128, 192 and 256).

## Structure

The content of the different files of the repository will be described below:

- <ins>aes128.h</ins>: Declaration of the two functions, both encryption and decryption.
- <ins>aes128.c</ins>: All the functions of each stage of the rounds.
- <ins>utils.c</ins>: Functions for the generation of the SBox table and its inverse.
- <ins>tests</ins>: A folder with a test based on the following example: <a href="https://www.kavaliro.com/wp-content/uploads/2014/03/AES.pdf">AES.pdf</a>

## Test

If you want to test the proposed test, execute the following commands:

```bash
cd tests/
make test
```
