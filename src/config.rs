use ini::Ini;
use std::fs::File;
use std::io::Read;

/// Gru configuration
pub struct Config {
    /// Public key
    pub pubkey: String,
    /// Database path
    pub db_path: String,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config, ()> {
        let conf = Ini::load_from_file(path).expect("opening config file");

        let db_section = conf.section(Some("database")).unwrap();
        let ssh_section = conf.section(Some("ssh")).unwrap();

        let mut pubkey = String::new();
        if let Some(key) = ssh_section.get("pubkey") {
            pubkey = key.to_owned();
        } else {
            let mut file = File::open(ssh_section.get("pubkey-path").unwrap()).expect("load public key");
            file.read_to_string(&mut pubkey).expect("read public key");
        }

        let db_path = db_section.get("path").unwrap().to_owned();

        Ok(Config {
            pubkey: pubkey,
            db_path: db_path,
        })
    }
}
