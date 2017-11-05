use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn connect() -> Pool {
    dotenv().ok();

    let config = r2d2::Config::default();

    
    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    r2d2::Pool::new(config, manager).expect("Could not initiate DB pool")
}
