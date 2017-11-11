#![feature(plugin)]
#![plugin(rocket_codegen)]

// Allow to use infer_schema macro
// See https://github.com/diesel-rs/diesel/issues/1127
#![recursion_limit="128"]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate clap;
extern crate rand;
extern crate base64;
extern crate rocket;
extern crate r2d2;
extern crate r2d2_diesel;

mod db;
mod cli;
mod models;
mod schema;
mod server;
mod manage_minions;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = cli::get_app(APP_NAME, VERSION).get_matches();

    let pool = db::connect();

    if matches.subcommand_matches("list").is_some() {
        let connection = pool.get().unwrap();
        manage_minions::list_minions(&connection);
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        let connection = pool.get().unwrap();
        manage_minions::create_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let connection = pool.get().unwrap();
        manage_minions::delete_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("revoke") {
        let connection = pool.get().unwrap();
        manage_minions::revoke_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("regenerate") {
        let connection = pool.get().unwrap();
        manage_minions::regen_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if matches.subcommand_matches("serve").is_some() {
        server::serve(pool);
    }
}
