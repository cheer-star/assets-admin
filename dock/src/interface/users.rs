/** 这一层不做单元测试，直接在上一层做集成测试 */
use crate::{
    database::users::{self, user_exist, user_insert, user_select, User},
    utils::{password_encrypt, password_verify},
};
use rocket::form::Form;
use rocket::serde::json::Json;

use super::response::Failed;

#[post("/create_user", data = "<user>")]
pub async fn create_user(user: Form<User>) -> Result<(), Json<Failed>> {
    let has_user = user_exist(user.account.to_string()).await;

    if has_user {
        let e = Failed {
            message: String::from("[user] already has this user."),
        };
        return Err(Json(e));
    }

    let password_after_process = password_encrypt(user.password.to_owned());

    let user_after_process = users::User {
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
pub async fn login(user: Form<User>) -> Result<(), Json<Failed>> {
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
        // jwt
        return Ok(());
    } else {
        let e = Failed {
            message: String::from("[user] password is incorrect."),
        };
        return Err(Json(e));
    }
}
