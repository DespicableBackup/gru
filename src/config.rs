use ini::Ini;
use std::fs::File;
use std::io::Read;

#[derive(Fail, Debug)]
#[fail(display = "Invalid configuration: {}", message)]
pub struct ConfigError {
    message: String,
}

/// Gru configuration
pub struct Config {
    /// Public key
    pub pubkey: String,
    /// Database path
    pub db_path: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, ConfigError> {
        let conf = Ini::load_from_file(path).map_err(|e| ConfigError { message: e.msg })?;

        let db_section = conf.section(Some("database"))
            .ok_or_else(|| ConfigError { message: "missing database section".to_owned() })?;
        let ssh_section = conf.section(Some("ssh"))
            .ok_or_else(|| ConfigError { message: "missing ssh section".to_owned() })?;

        let mut pubkey = String::new();
        if let Some(key) = ssh_section.get("pubkey") {
            pubkey = key.to_owned();
        } else {
            let mut file = File::open(
                    ssh_section.get("pubkey-path")
                    .ok_or_else(|| ConfigError { message: "missing pubkey config".to_owned() })?
                )
                .map_err(|_| ConfigError { message: "could not open public key file".to_owned() })?;
            file.read_to_string(&mut pubkey)
                .map_err(|_| ConfigError { message: "could not read public key".to_owned() })?;
        }

        let db_path = db_section.get("path")
            .ok_or_else(|| ConfigError { message: "missing database path".to_owned() })?
            .to_owned();

        Ok(Config {
            pubkey: pubkey,
            db_path: db_path,
        })
    }
}
