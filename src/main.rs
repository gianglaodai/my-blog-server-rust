use actix_web::{App, HttpServer};
use actix_web::web::Data;
use dotenv::dotenv;
use my_blog_server_svelte::config::app_state::AppState;
use my_blog_server_svelte::config::config::{get_config, init_pool};
use my_blog_server_svelte::routers::router::init_routers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = get_config();
    let pool = init_pool(&config).await.expect("Failed to create pool.");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { pool: pool.clone() }))
            .configure(init_routers)
    })
    .bind(("127.0.0.1", config.port))?
    .run()
    .await
}
