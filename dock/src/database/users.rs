use super::get_database_pool;
use sqlx;

pub async fn get_users() {
    let pool = get_database_pool().await.expect("msg");
    let row:sqlx::postgres::PgRow  = sqlx::query("select * from users;").fetch_one(&pool).await.expect("msg");

    println!("{:?}", row)
}
