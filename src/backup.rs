use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn backup_database(output_path: &str, db_url: &str) -> Result<(), Box<dyn Error>> {
    let output = Command::new("pg_dump")
        .arg("--dbname")
        .arg(db_url)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "pg_dump failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ).into());
    }

    let mut file = File::create(output_path)?;
    file.write_all(&output.stdout)?;

    Ok(())
}