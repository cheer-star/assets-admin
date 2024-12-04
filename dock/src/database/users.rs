use std::fmt::Debug;

use sqlx::postgres::PgQueryResult;

use super::get_database_pool;

#[derive(FromForm, sqlx::FromRow, Debug, PartialEq, Eq)]
pub struct User {
    // 标识值
    account: String,
    password: String,
}

/** 在数据处理阶段，不要对输入的数据作处理 */
pub async fn user_insert(user: User) -> Result<PgQueryResult, sqlx::Error> {
    let pool = get_database_pool()
        .await
        .expect("connect database failed in user_insert.");

    let result = sqlx::query("insert into users (account, password) values($1, $2);")
        .bind(user.account)
        .bind(user.password)
        .execute(&pool)
        .await
        .expect("insert user error.");

    println!("{:?}", result);
    Ok(result)
}

pub async fn user_exist(account: String) -> Result<bool, sqlx::Error> {
    let pool = get_database_pool()
        .await
        .expect("connect database failed in user_insert.");

    let result = sqlx::query("select 1 from users where account = $1 limit 1;")
        .bind(account)
        .fetch_one(&pool)
        .await
        .expect("msg");

    println!("this user_exist.{:?}", result);
    Ok(true)
}

/** test */
#[cfg(test)]
mod tests {
    use crate::database::users::user_exist;

    #[tokio::test]
    async fn test_user_exist() {
        let _ = user_exist("mapinxue".to_owned()).await;
    }
}
