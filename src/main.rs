mod sec;
mod cli;
mod backup;
mod data_crypto;
fn main() {

    let args = cli::parse_args();

    if let Some(password) = args.password {
        match sec::hash_password(&password) {
            Ok(hashed_password) => {
                println!("Hashed password: {}", hashed_password);
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if let (Some(key_str), Some(data_str)) = (args.key, args.data) {
        if key_str.len() != 16 {
            eprintln!("Error");
            return;
        }
        let key: [u8; 16] = key_str.as_bytes().try_into().unwrap();
        
        if args.decrypt {
            let decrypted_data = data_crypto::decrypt_data(&key, &data_str);
            println!("Decrypted data: {}", decrypted_data);
        } else {
            let encrypted_data = data_crypto::encrypt_data(&key, data_str.as_bytes());
            println!("Encrypted data: {}", encrypted_data);
        }
    }

    if let (Some(db_url), Some(output_path)) = (args.dbt_url, args.output_path) {
        match backup::backup_database(&output_path, &db_url) {
            Ok(_) => println!("Backup successful, saved to {}", output_path),
            Err(e) => eprintln!("Backup failed: {}", e),
        }
    }
}