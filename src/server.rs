use rocket;

use db::Pool;

#[post("/register")]
fn register() -> &'static str {
    "OK"
}

#[post("/unregister")]
fn unregister() -> &'static str {
    "Fine"
}

pub fn serve(pool: Pool) {
    rocket::ignite()
        .mount("/", routes![register, unregister])
        .launch();
}
