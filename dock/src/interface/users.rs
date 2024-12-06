use crate::database::users::{user_exist, user_insert, User};
use rocket::form::Form;

#[post("/create_user", data = "<user>")]
pub async fn create_user(user: Form<User>) -> &'static str {
    panic!("test world");

    let has_user = user_exist(user.account.to_string()).await;

    if has_user {
        return  "already has this user.";
    }

    // 密码加密

    let result = user_insert(user.into_inner()).await;
    
    if result {
        return "already create user.";
    }
    "hello"
}

#[post("/login", data = "<user>")]
pub async fn login(user: Form<User>) -> &'static str {
    println!("{:?}", user);
    ""
}
