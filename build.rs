use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Write;

const CONFIG_PATH: Option<&str> = option_env!("GRU_CONFIG_ENV");

fn main() {
    println!("cargo:rerun-if-env-changed=GRU_CONFIG_ENV");
    if let None = CONFIG_PATH {
        if let Ok(profile) = env::var("PROFILE") {
            println!("cargo:rustc-cfg=build={:?}", profile);
            if profile == "debug" || profile == "test" {
                if !Path::new("./dev.conf").exists() {
                    let mut f = File::create("./dev.conf").unwrap();
                    f.write_all(b"[database]
path=test.db
[ssh]
pubkey=testpubkey").unwrap();
                }
                println!("cargo:rustc-env=GRU_CONFIG_PATH=./dev.conf");
            } else {
                // Release conf
                println!("cargo:rustc-env=GRU_CONFIG_PATH=/etc/gru/server.conf");
            }
        }
    }
}
