use sqlx::{MySql, Pool};
use sqlx::mysql::MySqlPoolOptions;

pub async fn connect_pool() -> Pool<MySql> {
    MySqlPoolOptions::new().connect("mysql://root:root@127.0.0.1:3306/axum")
        .await
        .expect("Failed to connect to MySQL")
}
