use std::env;

use dotenv::dotenv;
use serde::Deserialize;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub database: String,
    pub host: String,
    pub db_port: u16,
    pub username: String,
    pub password: String,
    pub max_connection: u32,
}

pub fn get_config() -> Config {
    dotenv().ok();
    Config {
        port: env::var("PORT")
            .expect("PORT must be set")
            .parse()
            .expect("PORT must be number"),
        database: env::var("DATABASE").expect("DATABASE must be set"),
        host: env::var("DB_HOST").expect("DB_HOST must be set"),
        db_port: env::var("DB_PORT")
            .expect("DB_PORT must be set")
            .parse()
            .expect("DB_PORT must be number"),
        username: env::var("DB_USERNAME").expect("DB_USERNAME must be set"),
        password: env::var("DB_PASSWORD").expect("DB_PASSWORD must be set"),
        max_connection: env::var("MAX_CONNECTION")
            .expect("MAX_CONNECTION must be set")
            .parse()
            .expect("MAX_CONNECTION must be a number"),
    }
}

pub async fn init_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    println!("postgres://{}:{}@{}:{}/{}", &config.username, &config.password, &config.host, &config.db_port, &config.database);
    PgPoolOptions::new()
        .max_connections(config.max_connection)
        .connect(
            format!(
                "postgres://{}:{}@{}:{}/{}",
                config.username, config.password, config.host, config.db_port, config.database
            )
            .as_str(),
        )
        .await
}
