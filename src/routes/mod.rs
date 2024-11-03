use axum::Router;

mod user_routes;

pub fn get_routes() -> Router {
    Router::new()
}