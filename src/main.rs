#![feature(plugin)]
#![plugin(rocket_codegen)]

// Allow to use infer_schema macro
// See https://github.com/diesel-rs/diesel/issues/1127
#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate clap;
extern crate rand;
extern crate base64;
extern crate rocket;
extern crate rocket_contrib;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate ini;
#[macro_use] extern crate serde_derive;

mod db;
mod cli;
mod models;
mod schema;
mod server;
mod manage_minions;

use std::fs::File;
use std::io::Read;
use ini::Ini;

embed_migrations!();

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH: &str = env!("GRU_CONFIG_PATH");

fn main() {
    let matches = cli::get_app(APP_NAME, VERSION).get_matches();

    let conf = Ini::load_from_file(CONFIG_PATH).expect("Could not load config file");

    let db_section = conf.section(Some("database")).unwrap();

    let pool = db::connect(db_section.get("path").unwrap());

    let connection = pool.get().unwrap();
    match embedded_migrations::run(&*connection) {
        Ok(_) => {},
        Err(_) => {
            println!("Unable to run database migration");
            return;
        }
    }

    if matches.subcommand_matches("list").is_some() {
        manage_minions::list_minions(&connection);
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        manage_minions::create_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        manage_minions::delete_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("revoke") {
        manage_minions::revoke_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("regenerate") {
        manage_minions::regen_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("serve") {
        let mut file = File::open(matches.value_of("PUBKEY").unwrap()).expect("load public key");
        let mut pubkey = String::new();
        file.read_to_string(&mut pubkey).expect("read public key");
        server::serve(pool, pubkey);
    }
}
