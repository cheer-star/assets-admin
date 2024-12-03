pub mod users;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/**
 * 这是一个获取数据库连接池的全局函数
 * fixme: 数据库类型、数据库链接 都由 环境变量 决定
 * 函数重载
 */
pub async fn get_database_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mapinxue@localhost/assets")
        .await?;

    Ok(pool)
}
