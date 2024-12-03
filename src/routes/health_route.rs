use actix_web::{get, web::Data, HttpResponse};

use crate::{controllers::health_controller, AppState};

#[get("/health")]
async fn health_checker_handler(app_state: Data<AppState>) -> HttpResponse {
    return health_controller::get_health_check_info(app_state.db.clone()).await;
}