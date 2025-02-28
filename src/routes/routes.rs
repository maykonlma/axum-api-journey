use std::env;

use axum::Json;
use dotenvy::dotenv;

use crate::{config::config, dtos::user_dto};

pub async fn get_test() -> &'static str {
    "Hello, World!"
}

pub async fn get_system_env() -> String {
    match env::var("MY_SYSTEM_VARIABLE") {
        Ok(value) => value,
        Err(_) => String::from("Variável não encontrada."),
    }
}

pub async fn get_env() -> String {
    dotenv().ok();

    match env::var("MY_RUST_VARIABLE") {
        Ok(value) => value,
        Err(_) => String::from("Variável não encontrada."),
    }
}

pub async fn get_struct_env() -> String {
    dotenv().ok();

    let config = config::Config::new();
    config.my_rust_variable
}

pub async fn post_test(Json(user): Json<user_dto::User>) -> Json<user_dto::User> {
    Json(user_dto::User { name: user.name })
}
