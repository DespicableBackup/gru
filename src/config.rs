use toml;
use std::fs::File;
use std::io::Read;
use failure::Error;

#[derive(Fail, Debug)]
#[fail(display = "Invalid configuration: {}", message)]
pub struct ConfigError {
    message: String,
}

/// Config file to be deserialized
#[derive(Deserialize)]
struct ConfigValues {
    database: DbConfig,
    ssh: SshConfig,
}

#[derive(Deserialize)]
struct DbConfig {
    path: String,
}

#[derive(Deserialize)]
struct SshConfig {
    pubkey: Option<String>,
    #[serde(rename = "pubkey-path")]
    pubkey_path: Option<String>,
}

/// Gru configuration
pub struct Config {
    /// Public key
    pub pubkey: String,
    /// Database path
    pub db_path: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, Error> {
        let mut f = File::open(path)?;
        let mut buffer = String::new();
        f.read_to_string(&mut buffer)?;

        let conf: ConfigValues = toml::from_str(&buffer)?;

        let mut pubkey = String::new();
        if let Some(key) = conf.ssh.pubkey {
            pubkey = key.to_owned();
        } else {
            let mut keyfile = File::open(conf.ssh.pubkey_path.ok_or_else(|| ConfigError {
                message: "missing pubkey config".to_owned(),
            })?).map_err(|_| ConfigError {
                message: "could not open public key file".to_owned(),
            })?;
            keyfile.read_to_string(&mut pubkey).map_err(|_| ConfigError {
                message: "could not read public key".to_owned(),
            })?;
        }

        Ok(Config {
            pubkey: pubkey,
            db_path: conf.database.path,
        })
    }
}
