use rocket::http::Status;
use rocket::request::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use super::response::Failed;

#[derive(Deserialize, Debug, Serialize)]
pub struct Auth {
    pub account: String,
    pub role: String,
}

/**
 * 前端通过 http status code判断
*/
#[rocket::async_trait] 
impl<'a> FromRequest<'a> for Auth {
    type Error = Failed;

    async fn from_request(req: &'a Request<'_>) -> request::Outcome<Self, Self::Error> {
        let user_token = req.cookies().get_private("user_token");

        match user_token {
            Some(t) => {
                let user_raw = t.value();
                let user: Result<Auth, serde_json::Error> = serde_json::from_str(user_raw);

                match user {
                    Ok(u) => {
                        return Outcome::Success(u);
                    }
                    Err(error) => {
                        let e = Failed {
                            message: format!(
                                "[system] parse json error when read cookie. {:?}",
                                error.to_string()
                            ),
                        };
                        return Outcome::Error((Status::Unauthorized, e));
                    }
                }
            }
            None => {
                let e = Failed {
                    message: format!("unauthorized, please login."),
                };
                return Outcome::Error((Status::Unauthorized, e));
            }
        }
    }
}
