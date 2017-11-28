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
    let ssh_section = conf.section(Some("ssh")).unwrap();

    let pool = db::connect(db_section.get("path").unwrap());

    let connection = pool.get().unwrap();
    if embedded_migrations::run(&*connection).is_err() {
        println!("Unable to run database migration");
        return;
    }

    match matches.subcommand() {
        ("serve", _) => {
            let mut file = File::open(ssh_section.get("public-key").unwrap()).expect("load public key");
            let mut pubkey = String::new();
            file.read_to_string(&mut pubkey).expect("read public key");
            server::serve(pool, pubkey);
        },
        ("list", _) => manage_minions::list_minions(&connection),
        ("create", Some(args)) => {
            manage_minions::create_minion(&connection, args.value_of("NAME").unwrap());
        },
        ("delete", Some(args)) => {
            manage_minions::delete_minion(&connection, args.value_of("NAME").unwrap());
        },
        ("revoke", Some(args)) => {
            manage_minions::revoke_minion(&connection, args.value_of("NAME").unwrap());
        },
        ("regenerate", Some(args)) => {
            manage_minions::regen_minion(&connection, args.value_of("NAME").unwrap());
        },
        (_, _) => unimplemented!(),
    }
}
