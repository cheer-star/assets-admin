#[macro_use]
extern crate rocket;

mod database;
mod fairing;
mod interface;
mod utils;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(fairing::CORS)
        .mount("/assets", routes![interface::static_files])
        .mount("/", routes![interface::index])
        .mount(
            "/api",
            routes![
                interface::users::login,
                interface::users::create_user,
                interface::users::get_user_list
            ],
        )
}
