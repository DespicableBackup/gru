use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;

pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

pub fn connect(database_url: &str) -> Pool {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).unwrap()
}
