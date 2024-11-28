pub mod users;

#[get("/")]
pub async fn index() -> &'static str {
    "hello world"
}
