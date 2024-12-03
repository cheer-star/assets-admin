#[macro_use]
extern crate rocket;

// use rocket::fs::FileServer;

mod database;
mod interface;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/assets", routes![interface::static_files])
        .mount("/", routes![interface::index])
        .mount("/api", routes![interface::users::login])
}
