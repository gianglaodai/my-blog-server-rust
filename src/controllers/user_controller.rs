use actix_web::{get, HttpResponse, Responder};
use actix_web::web::Data;

use crate::config::app_state::AppState;
use crate::entities::user_entity::User;

#[get("/")]
pub async fn get_users(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&state.pool)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Application error: {e}");
            HttpResponse::NotFound().json("No users found")
        },
    }
}
