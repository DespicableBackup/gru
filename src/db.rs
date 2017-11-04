use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn connect() -> SqliteConnection {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
