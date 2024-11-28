#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
// use futures::executor::block_on;


use sqlx::postgres::PgPoolOptions;
// use sqlx::Row;

async fn db() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mapinxue@localhost/assets")
        .await?;

    let row = sqlx::query("select * from users;")
        .fetch_one(&pool)
        .await?;

    println!("any find");
    println!("Got: {:?}", row);

    Ok(())
}

#[get("/")]
async fn index() -> &'static str {
    println!("hello world");
    let res = db().await;

    let _ = res.inspect_err(|e| eprintln!("failed to read file: {e}"));

    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/public", FileServer::from("static"))
}
