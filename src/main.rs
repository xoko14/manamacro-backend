use axum::Router;
use axum::routing::get;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()>{
    let app = Router::new().route("/", get(|| async { "Hello, world!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
