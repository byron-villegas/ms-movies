use sysinfo::{Disks, System};

use crate::dtos::disk_space_dto::DiskSpaceDto;
use crate::dtos::health_response_dto::HealthResponseDto;
use crate::dtos::ram_space_dto::RamSpaceDto;

pub fn health_check_info() -> HealthResponseDto {
    let sys_info = System::new_all();

    let disks = Disks::new_with_refreshed_list();
    let disk = &disks[0];
    let disk_usage = (disk.total_space() - disk.available_space()) as i64;

    let ram_free = (sys_info.total_memory() - sys_info.used_memory()) as i64;

    let health_response_dto = HealthResponseDto {
        status: "UP".to_string(),
        diskSpace: DiskSpaceDto {
            status: "UP".to_string(),
            total: disk.total_space() as i64,
            free: disk.available_space() as i64,
            used: disk_usage,
        },
        ramSpace: RamSpaceDto {
            status: "UP".to_string(),
            total: sys_info.total_memory() as i64,
            free: ram_free,
            used: sys_info.used_memory() as i64,
        },
    };

    return health_response_dto;
}