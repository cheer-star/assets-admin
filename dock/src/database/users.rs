use super::get_database_pool;
use std::fmt::{format, Debug};

#[derive(FromForm, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    account: String,
    password: String,
}

/** 在数据处理阶段，不要对输入的数据作处理 */
pub async fn user_insert(user: User) -> Result<bool, sqlx::Error> {
    let pool = get_database_pool()
        .await
        .expect("connect database failed in user_insert.");

    let result = sqlx::query("insert into users (account, password) values($1, $2);")
        .bind(user.account)
        .bind(user.password)
        .execute(&pool)
        .await
        .expect("insert user error.");

    Ok(result.rows_affected() > 0)
}

struct Count {
    count: Option<i64>,
}

pub async fn user_exist(account: String) -> Result<bool, sqlx::Error> {
    let pool = get_database_pool()
        .await
        .expect("connect database failed in user_insert.");

    let result: i64 = sqlx::query_as!(
        Count,
        "select count(*) from users where account = $1 limit 1;",
        account
    )
    .fetch_one(&pool)
    .await
    .expect("user_exist error")
    .count
    .expect("get user count error");

    Ok(result > 0)
}

pub async fn user_select(account: Option<String>) -> Vec<User> {
    let pool = get_database_pool()
        .await
        .expect("connect database failed in user_insert.");

    let users_vec: Vec<User>;

    // 暂且如此实现，后续再优化
    match account {
        Some(acc) => {
            users_vec = sqlx::query_as!(User, "select * from users where account = $1;", acc)
                .fetch_all(&pool)
                .await
                .expect("user_exist error");
        }
        None => {
            users_vec = sqlx::query_as!(User, "select * from users;")
                .fetch_all(&pool)
                .await
                .expect("user_exist error");
        }
    }

    users_vec
}

/** test */
#[cfg(test)]
mod tests {
    use crate::database::users;

    #[tokio::test]
    async fn test_user_exist() {
        let _ = users::user_exist("test_user1".to_owned()).await;
    }

    #[ignore = "not need to create user."]
    #[tokio::test]
    async fn test_insert_user() {
        let user = users::User {
            account: "test_user1".to_owned(),
            password: "tes_user1".to_owned(),
        };
        let _ = users::user_insert(user).await;
    }

    #[tokio::test]
    async fn test_select_user() {
        let all_users = users::user_select(None).await;

        let user = users::user_select(Some("test_user1".to_string())).await;

        println!("{:?}", all_users);
        println!("{:?}", user);
    }
}
