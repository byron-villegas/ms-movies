use serde::Serialize;

use super::database_status_dto::DatabaseStatusDto;
use super::disk_space_dto::DiskSpaceDto;
use super::ram_space_dto::RamSpaceDto;

#[derive(Serialize)]
pub struct HealthResponseDto {
    pub status: String,
    #[serde(rename = "diskSpace")]
    pub disk_space: DiskSpaceDto,
    #[serde(rename = "ramSpace")]
    pub ram_space: RamSpaceDto,
    pub db: DatabaseStatusDto
}