use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};


pub fn encrypt_aes256(plaintext: &Vec<u8>, key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    
    // プレーンテキストを16バイトブロックにパディング
    let mut padded_text = plaintext.clone();
    
    // PKCS7パディングを実装
    let padding_len = 16 - (padded_text.len() % 16);
    for _ in 0..padding_len {
        padded_text.push(padding_len as u8);
    }
    
    let mut encrypted = Vec::new();
    
    // 16バイトずつ暗号化
    for chunk in padded_text.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.encrypt_block(&mut block);
        encrypted.extend_from_slice(&block);
    }
    
    encrypted
}

pub fn decrypt_aes256(ciphertext: &[u8], key: &[u8; 32]) -> String {
    let cipher = Aes256::new(GenericArray::from_slice(key));
    
    let mut decrypted = Vec::new();
    
    // 16バイトずつ復号化
    for chunk in ciphertext.chunks(16) {
        let mut block = GenericArray::clone_from_slice(chunk);
        cipher.decrypt_block(&mut block);
        decrypted.extend_from_slice(&block);
    }
    
    // パディングを削除
    if let Some(&padding_len) = decrypted.last() {
        if padding_len > 0 && padding_len <= 16 {
            let len = decrypted.len();
            decrypted.truncate(len - padding_len as usize);
        }
    }
    
    String::from_utf8(decrypted).unwrap_or_else(|_| "Invalid UTF-8".to_string())
}