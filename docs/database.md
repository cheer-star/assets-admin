# 关于数据库的组织

## 数据库的选择

默认选择了PostgreSQL，是因为这是一个`新开发`的项目。psql在许多方面都更有优势，尤其是在企业级别的频繁输入输出需求下。

## Rust连接数据库

没有选择使用ORM，而是使用了sqlX来管理数据库操作。

## 数据库的切换方式

这一点使用sqlX可以轻松实现，由于sqlX的设计类似于工厂模式，可以通过注入不同的连接池对象在不同的数据库中查询。其设计的代码如下:

```rust
let row = sqlx::query("select * from users;")
    .fetch_one(&pool) // 此处的pool就是数据库连接池
    .await?;
```
所以通过环境变量或其他方式选择不同的数据库连接池，将其交给sqlx的操作代码，实现可以在不变更代码的情况下更换数据库。实现数据库连接池的操作如下：
```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:mapinxue@localhost/assets")
    .await?;
```

> ⚠ 在你没有良好的DBA经验和网络安全管理经验的情况下，请保持你的数据库只能在本地访问。
