#include <stdint.h>

void initialize_aes_sbox(uint8_t sbox[256]) {
  #define ROTL8(x,shift) ((uint8_t) ((x) << (shift)) | ((x) >> (8 - (shift))))
	uint8_t p = 1, q = 1;
	uint8_t xformed;

	do {
		p = p ^ (p << 1) ^ (p & 0x80 ? 0x1B : 0);
		q ^= q << 1;
		q ^= q << 2;
		q ^= q << 4;
		q ^= q & 0x80 ? 0x09 : 0;

		xformed = q ^ ROTL8(q, 1) ^ ROTL8(q, 2) ^ ROTL8(q, 3) ^ ROTL8(q, 4);
		sbox[p] = xformed ^ 0x63;
	} while (p != 1);
	sbox[0] = 0x63;
}

void initialize_aes_inv_sbox(uint8_t inv_sbox[256], uint8_t sbox[256]) {
  for (short i = 0; i < 256; i++) {
    inv_sbox[sbox[i]] = (uint8_t)i;
  }
}
