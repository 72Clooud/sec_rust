use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use hex;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub fn encrypt_data(key: &[u8; 16], data: &[u8]) -> String {
    let mut iv = [0u8; 16];
    rand::thread_rng().fill(&mut iv);

    let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
    let ciphertext = cipher.encrypt_vec(data);

    let iv_hex = hex::encode(iv);
    let ciphertex_hex = hex::encode(ciphertext);

    format!("{}:{}", iv_hex, ciphertex_hex)
}

pub fn decrypt_data(key: &[u8; 16], encrypted_data: &str) -> String {
    let parts: Vec<&str> = encrypted_data.split(':').collect();
    if parts.len() != 2 {
        return String::from("Error: Invalid encrypted data format");
    }
    else {
       let iv_hex = parts[0];
       let ciphertext_hex = parts[1];

       let iv = hex::decode(iv_hex).unwrap();
       let ciphertext = hex::decode(ciphertext_hex).unwrap();

       let cipher = Aes128Cbc::new_from_slices(key, &iv).unwrap();
       let decrypted_data = cipher.decrypt_vec(&ciphertext).unwrap();

       String::from_utf8(decrypted_data).unwrap()
    }
}