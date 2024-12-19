use serde::Serialize;

#[derive(Serialize)]
pub struct DiskSpaceDto {
    pub status: String,
    pub total: i64,
    pub free: i64,
    pub used: i64
}