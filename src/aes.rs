use crate::tables;

/// Apply SubBytes transformation
fn sub_bytes(block: &mut [u8; 16]) {
    for i in 0..16 {
        block[i] = tables::SBOX[block[i] as usize];
    }
}

/// Apply inverse SubBytes transformation
fn inv_sub_bytes(block: &mut [u8; 16]) {
    for i in 0..16 {
        block[i] = tables::INV_SBOX[block[i] as usize];
    }
}

/// Apply ShiftRows transformation
fn shift_rows(block: &mut [u8; 16]) {
    // Row 1: shift left by 1
    let temp = block[1];
    block[1] = block[5];
    block[5] = block[9];
    block[9] = block[13];
    block[13] = temp;

    // Row 2: shift left by 2
    let temp = block[2];
    block[2] = block[10];
    block[10] = temp;
    let temp = block[6];
    block[6] = block[14];
    block[14] = temp;

    // Row 3: shift left by 3
    let temp = block[15];
    block[15] = block[11];
    block[11] = block[7];
    block[7] = block[3];
    block[3] = temp;
}

/// Apply inverse ShiftRows transformation
fn inv_shift_rows(block: &mut [u8; 16]) {
    // Row 1: shift right by 1
    let temp = block[13];
    block[13] = block[9];
    block[9] = block[5];
    block[5] = block[1];
    block[1] = temp;

    // Row 2: shift right by 2
    let temp = block[10];
    block[10] = block[2];
    block[2] = temp;
    let temp = block[14];
    block[14] = block[6];
    block[6] = temp;

    // Row 3: shift right by 3
    let temp = block[3];
    block[3] = block[7];
    block[7] = block[11];
    block[11] = block[15];
    block[15] = temp;
}

/// Apply MixColumns transformation
fn mix_columns(block: &mut [u8; 16]) {
    for col in 0..4 {
        let base = col * 4;
        let a0 = block[base];
        let a1 = block[base + 1];
        let a2 = block[base + 2];
        let a3 = block[base + 3];

        // Optimized multiplication by 2 in GF(2^8)
        let b0 = if a0 & 0x80 != 0 { (a0 << 1) ^ 0x1b } else { a0 << 1 };
        let b1 = if a1 & 0x80 != 0 { (a1 << 1) ^ 0x1b } else { a1 << 1 };
        let b2 = if a2 & 0x80 != 0 { (a2 << 1) ^ 0x1b } else { a2 << 1 };
        let b3 = if a3 & 0x80 != 0 { (a3 << 1) ^ 0x1b } else { a3 << 1 };

        block[base] = b0 ^ a3 ^ a2 ^ (a1 ^ b1);
        block[base + 1] = b1 ^ a0 ^ a3 ^ (a2 ^ b2);
        block[base + 2] = b2 ^ a1 ^ a0 ^ (a3 ^ b3);
        block[base + 3] = b3 ^ a2 ^ a1 ^ (a0 ^ b0);
    }
}

/// Multiply two elements in GF(2^8)
fn mult_gf256(mut a: u8, mut b: u8) -> u8 {
    let mut result = 0;
    
    for _ in 0..8 {
        if b & 1 != 0 {
            result ^= a;
        }
        let high_bit = a & 0x80 != 0;
        a <<= 1;
        if high_bit {
            a ^= 0x1b;
        }
        b >>= 1;
    }    
    result
}

/// Apply inverse MixColumns transformation
fn inv_mix_columns(block: &mut [u8; 16]) {
    for col in 0..4 {
        let base = col * 4;
        let a0 = block[base];
        let a1 = block[base + 1];
        let a2 = block[base + 2];
        let a3 = block[base + 3];

        block[base] = mult_gf256(0x0e, a0) ^ mult_gf256(0x0b, a1) ^ mult_gf256(0x0d, a2) ^ mult_gf256(0x09, a3);
        block[base + 1] = mult_gf256(0x09, a0) ^ mult_gf256(0x0e, a1) ^ mult_gf256(0x0b, a2) ^ mult_gf256(0x0d, a3);
        block[base + 2] = mult_gf256(0x0d, a0) ^ mult_gf256(0x09, a1) ^ mult_gf256(0x0e, a2) ^ mult_gf256(0x0b, a3);
        block[base + 3] = mult_gf256(0x0b, a0) ^ mult_gf256(0x0d, a1) ^ mult_gf256(0x09, a2) ^ mult_gf256(0x0e, a3);
    }
}

/// XOR block with round key
fn add_round_key(block: &mut [u8; 16], key: &[u8; 16]) {
    for i in 0..16 {
        block[i] ^= key[i];
    }
}

/// Apply g function for key expansion
fn g_function(word: &mut [u8; 4], round: usize) {
    // RotWord: rotate left by 1 byte
    let temp = word[0];
    word[0] = word[1];
    word[1] = word[2];
    word[2] = word[3];
    word[3] = temp;

    for i in 0..4 {
        // Apply S-box amd XOR with round constant
        word[i] = tables::SBOX[word[i] as usize];
        word[i] ^= tables::RCON[round][i];
    }
}

/// Expand key for one round
fn expand_key(key: &mut [u8; 16], round: usize) {
    let mut g_word = [key[12], key[13], key[14], key[15]];
    g_function(&mut g_word, round);

    for i in 0..4 {
        // Generate [1-4] words
        key[i] ^= g_word[i];
        key[4 + i]  = key[i] ^ key[4 + i];
        key[8 + i]  = key[4 + i] ^ key[8 + i];
        key[12 + i] = key[8 + i] ^ key[12 + i];
    }
}

/// Encrypt a single 128-bit block
pub fn encrypt_block(plaintext: [u8; 16], key: [u8; 16]) -> [u8; 16] {
    let mut block = plaintext;
    let mut round_key = key;

    // Initial round key addition
    add_round_key(&mut block, &round_key);

    // 10 rounds
    for round in 0..tables::NUM_ROUNDS {
        sub_bytes(&mut block);
        shift_rows(&mut block);
        
        if round < tables::NUM_ROUNDS - 1 {
            mix_columns(&mut block);
        }
        
        expand_key(&mut round_key, round);
        add_round_key(&mut block, &round_key);
    }
    block
}

/// Decrypt a single 128-bit block
pub fn decrypt_block(cipher: [u8; 16], key: [u8; 16]) -> [u8; 16] {
    let mut block = cipher;
    
    // Generate all round keys first
    let mut round_keys = [[0u8; 16]; tables::NUM_ROUNDS];
    let mut current_key = key;
    
    for round in 0..tables::NUM_ROUNDS {
        expand_key(&mut current_key, round);

        if round < tables::NUM_ROUNDS - 1 {
            round_keys[round] = current_key;
        }
    }

    // Initial round key addition (last round key)
    add_round_key(&mut block, &current_key);

    // 10 rounds in reverse
    for round in 0..tables::NUM_ROUNDS {
        inv_shift_rows(&mut block);
        inv_sub_bytes(&mut block);
        
        if round == tables::NUM_ROUNDS - 1 {
            break;
        }
        
        add_round_key(&mut block, &round_keys[tables::NUM_ROUNDS - 2 - round]);
        inv_mix_columns(&mut block);
    }

    add_round_key(&mut block, &key);
    cipher
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = [
            0x2b, 0x7e, 0x15, 0x16,
            0x28, 0xae, 0xd2, 0xa6,
            0xab, 0xf7, 0x15, 0x88,
            0x09, 0xcf, 0x4f, 0x3c,
        ];
        
        let plaintext = [
            0x32, 0x43, 0xf6, 0xa8,
            0x88, 0x5a, 0x30, 0x8d,
            0x31, 0x31, 0x98, 0xa2,
            0xe0, 0x37, 0x07, 0x34,
        ];

        let ciphertext = encrypt_block(plaintext, key);
        let decrypted = decrypt_block(ciphertext, key);
        assert_eq!(plaintext, decrypted);
    }

    #[test]
    fn test_nist_vector() {
        // NIST test vector
        let key = [0u8; 16];
        let plaintext = [0u8; 16];
        let expected_ciphertext = [
            0x66, 0xe9, 0x4b, 0xd4,
            0xef, 0x8a, 0x2c, 0x3b,
            0x88, 0x4c, 0xfa, 0x59,
            0xca, 0x34, 0x2b, 0x2e
        ];

        let ciphertext = encrypt_block(plaintext, key);
        assert_eq!(ciphertext, expected_ciphertext);
    }
}
