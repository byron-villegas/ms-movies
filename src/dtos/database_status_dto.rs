use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct DatabaseStatusDto {
    pub status: String,
    pub database: String
}