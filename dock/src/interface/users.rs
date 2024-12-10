/** 这一层不做单元测试，直接在上一层做集成测试 */
use crate::{
    database::users::{self, user_exist, user_insert, user_select, User},
    utils::{password_encrypt, password_verify},
};
use rocket::serde::json::Json;
use rocket::{form::Form, time::Duration};

use super::{guard::Auth, response::Failed};
use rocket::http::{Cookie, CookieJar};

#[post("/create_user", data = "<user>")]
pub async fn create_user(user: Form<User>, a: Auth) -> Result<(), Json<Failed>> {
    if !a.role.eq("admin") {
        let e = Failed {
            message: String::from("[user] only administrators can create user."),
        };
        return Err(Json(e));
    }

    let has_user = user_exist(user.account.to_string()).await;

    if has_user {
        let e = Failed {
            message: String::from("[user] already has this user."),
        };
        return Err(Json(e));
    }

    let password_after_process = password_encrypt(user.password.to_owned());

    let user_after_process: User = users::User {
        account: user.account.to_owned(),
        password: password_after_process,
    };

    let result = user_insert(user_after_process).await;

    if result {
        return Ok(());
    } else {
        let s = Failed {
            message: "[database] insert user failed with a internal reason.".to_owned(),
        };
        return Err(Json(s));
    }
}

#[post("/login", data = "<user>")]
pub async fn login(user: Form<User>, cookies: &CookieJar<'_>) -> Result<(), Json<Failed>> {
    let u = user_select(Some(user.account.to_owned())).await;

    if u.len() == 0 {
        let e = Failed {
            message: String::from("[user] user is not exist."),
        };
        return Err(Json(e));
    }

    let user_from_db = &u[0];
    let password_correct =
        password_verify(user.password.to_owned(), user_from_db.password.to_string());

    if password_correct {
        let auth = Auth {
            account: user.account.to_owned(),
            role: String::from("admin"),
        };

        let auth_json = serde_json::to_string(&auth);

        match auth_json {
            Ok(res) => {
                let user_cookie = Cookie::build(("user_token", res))
                    .path("/")
                    .http_only(false)
                    .max_age(Duration::days(3))
                    .secure(false);
                cookies.add_private(user_cookie);
                return Ok(());
            }
            Err(e) => {
                let e = Failed {
                    message: format!(
                        "[system] Authority to json string error. {:?}",
                        e.to_string()
                    ),
                };
                return Err(Json(e));
            }
        }
    } else {
        let e = Failed {
            message: String::from("[user] password is incorrect."),
        };
        return Err(Json(e));
    }
}
