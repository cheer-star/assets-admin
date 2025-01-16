/** 数据库只做数据库操作，逻辑功能等放到上一层 */
pub mod users;
use sqlx::{postgres::PgPoolOptions, Pool};

/**
 * 这是一个获取数据库连接池的全局函数
 * fixme: 数据库类型、数据库链接 都由 环境变量 决定
 * 函数重载
 */
pub async fn get_database_pool() -> Result<Pool<sqlx::Postgres>, sqlx::Error> {
    let pool: Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mapinxue@localhost/assets")
        .await?;

    Ok(pool)
}

pub async fn get_sqlite_pool() -> Result<Pool<sqlx::Sqlite>, sqlx::Error> {
    let local_db_pool = sqlx::sqlite::SqlitePool::connect("sqlite:local.db").await?;

    Ok(local_db_pool)
}
