use axum::{
    routing::{get, post},
    Router,
};

mod config;
mod dtos;
mod routes;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(routes::routes::get_test))
        .route("/system_env", get(routes::routes::get_system_env))
        .route("/env", get(routes::routes::get_env))
        .route("/struct_env", get(routes::routes::get_struct_env))
        .route("/", post(routes::routes::post_test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
