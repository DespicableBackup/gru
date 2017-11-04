#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate clap;

mod db;
mod cli;
mod models;
mod schema;

use diesel::prelude::*;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn create_minion(conn: &SqliteConnection, name: &str) {
    use schema::minions;

    let minion = models::NewMinion {
        name: name,
    };

    diesel::insert(&minion)
        .into(minions::table)
        .execute(conn)
        .expect("Error saving new minion");
}

fn list_minions(conn: &SqliteConnection) {
    use self::schema::minions::dsl::*;
    let results :Vec<models::Minion> = minions.load(conn).expect("Could not retrieve minions");

    for minion in results {
        println!("{}", minion.name);
    }
}

fn main() {
    let matches = cli::get_app(APP_NAME, VERSION).get_matches();

    let connection = db::connect();

    if let Some(_) = matches.subcommand_matches("list") {
        list_minions(&connection);
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        create_minion(&connection, matches.value_of("NAME").unwrap());
    }
}
