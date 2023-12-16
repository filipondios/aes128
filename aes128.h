#ifndef _AES128_H
#define _AES128_H
#include <stdint.h>

void aes128_encrypt(const uint8_t block[16], uint8_t cipher_block[16], const uint8_t key[16]);
void aes128_decrypt(const uint8_t cipher_block[16], uint8_t block[16], const uint8_t key[16]);

#endif // !_AES128_H
