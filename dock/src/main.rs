#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use sqlx::postgres::PgPoolOptions;

// mod database;
mod interface;

async fn db() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mapinxue@localhost/assets")
        .await?;

    let row = sqlx::query("select * from users;").fetch_one(&pool).await?;

    println!("any find");
    println!("Got: {:?}", row);

    // init::get_database();

    Ok(())
}

#[launch]
fn rocket() -> _ {
    // mount 后面的路径是 /login  /注释的路径的组合
    rocket::build()
        .mount("/", routes![interface::index])
        .mount("/login", routes![interface::users::login])
        .mount("/public", FileServer::from("static"))
}
