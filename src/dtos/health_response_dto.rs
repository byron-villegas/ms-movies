use serde::Serialize;

use super::database_status_dto::DatabaseStatusDto;
use super::disk_space_dto::DiskSpaceDto;
use super::ram_space_dto::RamSpaceDto;

#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct HealthResponseDto {
    pub status: String,
    pub diskSpace: DiskSpaceDto,
    pub ramSpace: RamSpaceDto,
    pub db: DatabaseStatusDto
}