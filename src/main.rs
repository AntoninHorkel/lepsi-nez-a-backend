use std::error::Error;

use axum::{Router, routing};
// use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new().route("/", routing::get(|| async { "Hello, World!" }));
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    axum::serve(listener, router).await?;
    // let pg_pool = PgPool::connect(todo!()).await?;
    Ok(())
}
