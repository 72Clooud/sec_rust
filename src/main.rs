mod sec;
mod data_crypto;
use clap::{App, Arg};
fn main() {
    let matches = App::new("Sec App")
        .arg(Arg::new("password")
            .short('p')
            .long("password")
            .takes_value(true)
            .help("Password to be hashed")) 
        .arg(Arg::new("key")
            .short('k')
            .long("key")
            .takes_value(true)
            .help("16-byte encryption key"))
        .arg(Arg::new("data")
            .short('d')
            .long("data")
            .takes_value(true)
            .help("Data to encrypt or decrypt"))
        .arg(Arg::new("decrypt")
            .short('r')
            .long("decrypt")
            .help("Set this flag to decrypt instead of encrypt"))
        .get_matches();

    if let Some(password) = matches.value_of("password") {
        match sec::hash_password(password) {
            Ok(hashed_password) => {
                println!("Hashed password: {}", hashed_password);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if let (Some(key_str), Some(data_str)) = (matches.value_of("key"), matches.value_of("data")) {
        if key_str.len() != 16 {
            eprintln!("Error");
            return;
        }
        let key: [u8; 16] = key_str.as_bytes().try_into().unwrap();
        
        if matches.is_present("decrypt") {
            let decrypted_data = data_crypto::decrypt_data(&key, data_str);
            println!("Decrypted data: {}", decrypted_data);
        } else {
            let encrypted_data = data_crypto::encrypt_data(&key, data_str.as_bytes());
            println!("Encrypted data: {}", encrypted_data);
        }
    }
}