#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;

mod interface;
mod database;
mod env;


#[launch]
fn rocket() -> _ {
    // mount 后面的路径是 /login  /注释的路径的组合
    rocket::build()
        .mount("/", routes![interface::index])
        .mount("/login", routes![interface::users::login])
        .mount("/public", FileServer::from("static"))
}
