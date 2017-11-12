use std::ops::Deref;
use std::net::IpAddr;
use db::Pool;
use diesel;
use diesel::prelude::*;
use r2d2;
use r2d2_diesel::ConnectionManager;
use rocket;
use rocket_contrib::Json;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use models::Minion;

const API_KEY_HEADER: &str = "X-API-KEY";

struct DbConn(r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);
struct Ip(IpAddr);
struct Pubkey(String);

#[derive(Deserialize)]
struct Registration {
    username: String,
    port: i32,
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Ip {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Ip, ()> {
        match request.remote() {
            Some(addr) => Outcome::Success(Ip(addr.ip())),
            None => Outcome::Failure((Status::BadRequest, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Minion {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Minion, ()> {
        use schema::minions::dsl;

        let keys: Vec<_> = request.headers().get(API_KEY_HEADER).collect();
        if keys.len() != 1 {
            return Outcome::Failure((Status::Unauthorized, ()));
        }

        let key = keys[0];

        let pool = request.guard::<State<Pool>>()?;
        if let Ok(conn) = pool.get() {
            match dsl::minions.filter(dsl::key.eq(key)).first(&*conn) {
                Ok(minion) => Outcome::Success(minion),
                Err(_) => Outcome::Failure((Status::Forbidden, ()))
            }
        } else {
            Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

// Register a minion as active
#[post("/register", data="<input>")]
fn register(conn: DbConn, minion: Minion, ip: Ip, input: Json<Registration>, pubkey: State<Pubkey>) -> String {
    use schema::minions::dsl;

    diesel::update(&minion)
        .set((
                dsl::active.eq(true),
                dsl::ip.eq(format!("{}", ip.0)),
                dsl::username.eq(&input.username),
                dsl::port.eq(&input.port),
             ))
        .execute(&*conn)
        .expect(&format!("Could not update {}", &minion.name));
    // TODO: avoid clone
    pubkey.0.clone()
}

// Set a minion as inactive
#[post("/unregister")]
fn unregister(conn: DbConn, minion: Minion) {
    use schema::minions::dsl;

    diesel::update(&minion)
        .set((
                dsl::active.eq(false),
                dsl::ip.eq("")
             ))
        .execute(&*conn)
        .expect(&format!("Could not update {}", &minion.name));
}

pub fn serve(pool: Pool, pubkey: String) {
    rocket::ignite()
        .mount("/", routes![register, unregister])
        .manage(pool)
        .manage(Pubkey(pubkey))
        .launch();
}
