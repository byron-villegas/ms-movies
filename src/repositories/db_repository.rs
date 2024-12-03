
use sea_orm::DatabaseConnection;

pub async fn check(db: DatabaseConnection) -> bool {
    return db.ping().await.is_ok();
}
