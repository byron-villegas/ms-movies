use serde::Serialize;

#[derive(Serialize)]
pub struct DatabaseStatusDto {
    pub status: String,
    pub database: String
}