use actix_web::HttpResponse;
use actix_web::web::Data;
use sqlx::Error;
use crate::config::app_state::AppState;
use crate::entities::user_entity::User;
use crate::repositories::user_repository::find_users;
pub async fn get_users(state: Data<AppState>) -> HttpResponse {
    let find_users: fn() -> Result<Vec<User>, Error> = find_users(&state.pool);
    let users = find_users().await.unwrap();
    HttpResponse::Ok().json(users)
}
