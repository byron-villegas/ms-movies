use actix_web::{get, HttpResponse};

use crate::controllers::health_controller::get_health_check_info;

#[get("/health")]
async fn health_checker_handler() -> HttpResponse  {
    return get_health_check_info();
}