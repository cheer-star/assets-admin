use rocket::serde::Deserialize;

#[derive(Deserialize, serde::Serialize)]
pub struct Failed {
    pub message: String,
}
