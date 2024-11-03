use axum::{Json, Router};
use axum::response::Html;
use axum::routing::get;
use axum_swagger_ui::swagger_ui;
use utoipa::OpenApi;

#[derive(OpenApi)]
struct ApiDoc;

async fn openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}

async fn swagger() -> Html<String> {
    swagger_ui("/openapi.json").into()
}

pub fn get_openapi_routes() -> Router {
    Router::new()
        .route("/openapi.json", get(openapi))
        .route("/", get(swagger))
}
