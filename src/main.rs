#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;
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
mod config;

embed_migrations!();

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH: &str = env!("GRU_CONFIG_PATH");

fn main() {
    let matches = cli::get_app(APP_NAME, VERSION).get_matches();

    let conf = config::Config::from_file(CONFIG_PATH).expect("loading config file");

    let pool = db::connect(&conf.db_path);

    let connection = pool.get().unwrap();
    if embedded_migrations::run(&*connection).is_err() {
        println!("Unable to run database migration");
        return;
    }

    match matches.subcommand() {
        ("serve", _) => {
            server::serve(pool, &conf);
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
