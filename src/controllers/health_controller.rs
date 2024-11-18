use actix_web::HttpResponse;

use crate::services::health_service::health_check_info;

pub fn get_health_check_info() -> HttpResponse {
    let health_response_dto = health_check_info();

    HttpResponse::Ok().json(health_response_dto)
}