
mod tables;
mod aes;

pub fn main() {

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

    let cipher: [u8; 16] = aes::encrypt_block(block, key);
    let plain: [u8; 16] = aes::decrypt_block(cipher, key);

    println!("Encrypting block {} using key {} produces {}",
        block_str(&block), block_str(&key), block_str(&cipher));
   
    println!("Decrypting block {} using key {} produces {}",
        block_str(&cipher), block_str(&key), block_str(&plain));
}

pub fn block_str(block: &[u8; 16]) -> String {
    block.iter().map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("")    
}
