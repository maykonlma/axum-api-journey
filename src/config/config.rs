use std::env;

pub struct Config {
    pub my_rust_variable: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            my_rust_variable: env::var("MY_RUST_VARIABLE").expect("Variável não encontrada."),
        }
    }
}
