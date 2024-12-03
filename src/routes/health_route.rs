use actix_web::{get, web::Data, HttpResponse};

use crate::{controllers::health_controller::get_health_check_info, AppState};

#[get("/health")]
async fn health_checker_handler(app_state: Data<AppState>) -> HttpResponse  {
    let db: &sea_orm::DatabaseConnection = &app_state.db;

    return get_health_check_info(db.clone()).await;
}