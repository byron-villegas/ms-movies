use actix_web::HttpResponse;
use mongodb::Database;

use crate::services::health_service;

pub async fn get_health_check_info(db: Database) -> HttpResponse {
    let health_response_dto = health_service::health_check_info(db).await;

    HttpResponse::Ok().json(health_response_dto)
}