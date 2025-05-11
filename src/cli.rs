use clap::{Arg, ArgAction, Command};

pub struct Args {
    pub password: Option<String>,
    pub key: Option<String>,
    pub data: Option<String>,
    pub dbt_url: Option<String>,
    pub output_path: Option<String>,
    pub decrypt: bool,
}

pub fn parse_args() -> Args {
    let matches = Command::new("Sec App")
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
            .action(ArgAction::SetTrue)
            .help("Set this flag to decrypt instead of encrypt"))
        .arg(Arg::new("backup")
            .long("backup")
            .help("Run backup of the PostgreSQL database")
            .takes_value(false))
        .arg(Arg::new("db_url")
            .long("db_url")
            .takes_value(true)
            .help("Database connection URL for backup"))
        .arg(Arg::new("output_path")
            .long("output_path")
            .takes_value(true)
            .help("Path to output_path backup file"))
        .get_matches();

    Args {
        password: matches.get_one::<String>("password").cloned(),
        key: matches.get_one::<String>("key").cloned(),
        data: matches.get_one::<String>("data").cloned(),
        dbt_url: matches.get_one::<String>("db_url").cloned(),
        output_path: matches.get_one::<String>("output_path").cloned(),
        decrypt: matches.get_flag("decrypt"),
    }
}