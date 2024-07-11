use actix_web::web::{scope, ServiceConfig};
use crate::controllers::user_controller;

pub fn init_routers(config: &mut ServiceConfig) {
    config.service(
        scope("/users")
            .service(user_controller::get_users),
    );
}
