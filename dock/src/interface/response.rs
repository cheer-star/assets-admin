use rocket::serde::Deserialize;

#[derive(Deserialize, serde::Serialize, Debug)]
pub struct Failed {
    pub message: String,
}

#[derive(Deserialize, serde::Serialize, Debug)]
pub struct UserInfo {
    pub account: String,
}
