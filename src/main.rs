use axum::Router;
use anyhow::Result;
use crate::openapi::get_openapi_routes;
use crate::routes::get_routes;

mod models;
mod openapi;
mod routes;

#[tokio::main]
async fn main() -> Result<()>{
    let app = Router::new()
        .merge(get_openapi_routes())
        .merge(get_routes());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
