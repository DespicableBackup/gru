use diesel::prelude::*;
use rocket;

#[post("/register")]
fn register() -> &'static str {
    "OK"
}

#[post("/unregister")]
fn unregister() -> &'static str {
    "Fine"
}

pub fn serve(conn: &SqliteConnection) {
    rocket::ignite()
        .mount("/", routes![register, unregister])
        .launch();
}
