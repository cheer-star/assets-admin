pub mod users;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

/**
 * 这是一个获取数据库连接池的全局函数
 * 数据库类型、数据库链接 都由 环境变量 决定
 */
pub async fn get_database_pool() -> Result<Pool<Postgres>, sqlx::Error> {
    // fixme: 更改这里的代码，能够从环境变量中获取数据库连接信息
    // let db_type = get_env("DB_TYPE".to_owned());
    // let db_link = get_env("DB_LINK".to_owned());

    // let pool: ;
    // match db_type {
    //     _ => println!(""),
    // }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:mapinxue@localhost/assets")
        .await?;

    Ok(pool)
}
