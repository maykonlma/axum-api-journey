use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_word))
        .route("/", post(post_user));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello_word() -> &'static str {
    "Hello, World!"
}

async fn post_user(Json(user): Json<User>) -> Json<User> {
    Json(User {
        name: format!("Hello: {}", user.name),
    })
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String
}