use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct RamSpaceDto {
    pub status: String,
    pub total: i64,
    pub free: i64,
    pub used: i64,
}