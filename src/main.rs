#![feature(plugin)]
#![plugin(rocket_codegen)]

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

use diesel::prelude::*;
use rand::{OsRng, Rng};
use base64::encode;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const KEY_LENGTH: usize = 64;

fn generate_key() -> String {
    let mut rng = OsRng::new().expect("Could not get a proper random generator");
    let mut buf: Vec<u8> = vec![0; KEY_LENGTH];
    rng.fill_bytes(&mut buf);

    encode(&buf)
}

fn create_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions;

    let key = generate_key();

    let minion = models::NewMinion {
        name: name,
        key: &key,
    };

    diesel::insert(&minion)
        .into(minions::table)
        .execute(conn)
        .expect("Error saving new minion");

    println!("Minion API key: {}", key);
}

fn delete_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    diesel::delete(dsl::minions.filter(dsl::name.eq(name)))
        .execute(conn)
        .expect("Error deleting minion");
}

fn list_minions(conn: &SqliteConnection) {
    use schema::minions::dsl::*;
    let results :Vec<models::Minion> = minions.load(conn).expect("Could not retrieve minions");

    for minion in results {
        println!("{}", minion.name);
    }
}

fn revoke_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            active: Some(false),
            key: Some(None)
        })
        .execute(conn)
        .expect(&format!("Could not revoke {}", name));
}

fn regen_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions::dsl;

    let key = generate_key();

    diesel::update(dsl::minions.filter(dsl::name.eq(name)))
        .set(&models::UpdateMinion {
            // Disable the minion as the key may have been compromised
            active: Some(false),
            key: Some(Some(&key))
        })
        .execute(conn)
        .expect(&format!("Could not revoke {}", name));
    println!("New key: {}", key);
}

fn main() {
    let matches = cli::get_app(APP_NAME, VERSION).get_matches();

    let pool = db::connect();

    if matches.subcommand_matches("list").is_some() {
        let connection = pool.get().unwrap();
        list_minions(&connection);
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        let connection = pool.get().unwrap();
        create_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("delete") {
        let connection = pool.get().unwrap();
        delete_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("revoke") {
        let connection = pool.get().unwrap();
        revoke_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if let Some(matches) = matches.subcommand_matches("regenerate") {
        let connection = pool.get().unwrap();
        regen_minion(&connection, matches.value_of("NAME").unwrap());
    }

    if matches.subcommand_matches("serve").is_some() {
        server::serve(pool);
    }
}
