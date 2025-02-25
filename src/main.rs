use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::env;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_test))
        .route("/system_env", get(get_system_env))
        .route("/env", get(get_env))
        .route("/struct_env", get(get_struct_env))
        .route("/", post(post_test));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_test() -> &'static str {
    "Hello, World!"
}

async fn get_system_env() -> String {
    match env::var("MY_SYSTEM_VARIABLE") {
        Ok(value) => value,
        Err(_) => String::from("Variável não encontrada."),
    }
}

async fn get_env() -> String {
    dotenv().ok();

    match env::var("MY_RUST_VARIABLE") {
        Ok(value) => value,
        Err(_) => String::from("Variável não encontrada."),
    }
}

async fn get_struct_env() -> String {
    dotenv().ok();

    let config = Config::new();
    config.my_rust_variable
}

async fn post_test(Json(user): Json<User>) -> Json<User> {
    Json(User {
        name: user.name,
    })
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String
}

pub struct Config {
    pub my_rust_variable: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            my_rust_variable: env::var("MY_RUST_VARIABLE").expect("Variável não encontrada.")
        }
    }
}