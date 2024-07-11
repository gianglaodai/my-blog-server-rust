use std::env;
use std::fmt::format;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize)]
pub struct Config {
    pub port: String,
    pub database: String,
    pub host: String,
    pub db_port: String,
    pub username: String,
    pub password: String,
    pub max_connection: u32
}

pub fn get_config() -> Config {
    dotenv().ok();
    Config {
        port: env::var("PORT").expect("PORT must be set"),
        database: env::var("DATABASE").expect("DATABASE must be set"),
        host: env::var("HOST").expect("HOST must be set"),
        db_port: env::var("DB_PORT").expect("DB_PORT must be set"),
        username: env::var("USERNAME").expect("USERNAME must be set"),
        password: env::var("PASSWORD").expect("PASSWORD must be set"),
        max_connection: env::var("MAX_CONNECTION").expect("MAX_CONNECTION must be set").parse().expect("MAX_CONNECTION must be a number")
    }
}

pub async fn init_pool(config: &Config) -> Result <PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connection)
        .connect(format!("postgres://{}:{}@{}:{}/{}", config.username, config.password, config.host, config.db_port, config.database).as_str())
        .await
}