mod sec;
use clap::{App, Arg};
fn main() {
    let matches = App::new("Sec App")
        .arg(Arg::new("password")
            .short('p')
            .long("password")
            .takes_value(true)
            .help("Password to be hashed")) 
        .get_matches();

    let password = matches.value_of("password").unwrap_or_else(|| {
        eprintln!("Please provide a password using --password <password>");
        std::process::exit(1);
    });

    match sec::hash_password(password) {
        Ok(hashed_password) => {
            println!("Hashed password: {}", hashed_password);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}