use mongodb::Database;

pub async fn check(db: Database) -> bool {
    return !db.name().is_empty();
}