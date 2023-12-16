#include "../aes128.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(void) {
  
  /* Key to use */
  const uint8_t key[16] = { 
    0x54, 0x68, 0x61, 0x74,
    0x73, 0x20, 0x6D, 0x79,
    0x20, 0x4B, 0x75, 0x6E,
    0x67, 0x20, 0x46, 0x75, 
  };

  /* Block to encrypt */ 
  const uint8_t block[16] = {
    0x54, 0x77, 0x6F, 0x20,
    0x4F, 0x6E, 0x65, 0x20,
    0x4E, 0x69, 0x6E, 0x65,
    0x20, 0x54, 0x77, 0x6F,
  };

  uint8_t cipher_block[16];
  uint8_t recovered_block[16];
  
  aes128_encrypt(block, cipher_block, key);
  aes128_decrypt(cipher_block, recovered_block, key);

  if(memcmp(block, recovered_block, 16)){
    printf("AES failed.\n");
    exit(EXIT_FAILURE);
  } else {
    printf("AES succeeded.\n");
    exit(EXIT_SUCCESS);
  }
}
