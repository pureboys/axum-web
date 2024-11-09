mod handler;
mod pool;
mod utils;

use axum::{routing, Router};
use sqlx::{MySql, Pool};
use tokio::net::TcpListener;
use crate::handler::tickets::*;
use crate::pool::connect_pool;

#[derive(Debug, Clone)]
pub struct AppState {
    pool: Pool<MySql>,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt::init();

    let pool = connect_pool().await;

    let app = Router::new()
        .route("/", routing::get(root_handler))
        .route("/tickets", routing::post(create_handle))
        .route("/tickets/:id", routing::put(update_handle))
        .route("/tickets/:id", routing::delete(delete_handle))
        .route("/tickets/:id", routing::get(get_handle))
        .route("/tickets", routing::get(list_handle))
        .with_state(AppState { pool });

    let tcp_listener = TcpListener::bind("127.0.0.1:8888").await?;

    axum::serve(tcp_listener, app).await?;
    Ok(())
}


