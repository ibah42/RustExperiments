use std::fmt::Debug;
use std::fs::{DirEntry, ReadDir};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use normpath::PathExt;
use crate::utils::time_utils;
use crate::file_commander::volume::Volume;
use crate::file_commander::disk_type::DiskType;


pub fn get_files(path: PathBuf) -> Vec<DirEntry>{

    let result: std::io::Result<ReadDir> = std::fs::read_dir(path);
    if result.is_err()  {
        return vec![];
    }

    let mut entries = Vec::<DirEntry>::with_capacity(128);

    for entry in result.unwrap() {
        if(entry.is_err()) {
            break;
        }
        entries.push(entry.unwrap());
    }

    return entries;
}

pub async fn get_volumes() -> Vec<Volume>
{
    let mut disks = sysinfo::Disks::new();
    disks.refresh_list();

    let mut result = futures_util::future::join_all(disks.iter().map(|disk| async {
        #[cfg(not(windows))]
        let disk_name = disk.name();
        let mount_point = disk.mount_point().to_path_buf().normalize_virtually().expect("Could not normalize mount point");

        let disk_name = mount_point.localize_name().to_os_string();
        let mount_point = mount_point.into_path_buf();
        let is_root_filesystem = mount_point.is_absolute() && mount_point.parent().is_none();

        let mut name = disk_name.to_string_lossy().to_string();
        if name.replace(char::REPLACEMENT_CHARACTER, "").is_empty() {
            name = "Unknown".to_string()
        }

        return Some(Volume {
            name,
            disk_type: if disk.is_removable() {
                DiskType::Removable
            } else {
                match disk.kind() {
                    sysinfo::DiskKind::SSD => DiskType::SSD,
                    sysinfo::DiskKind::HDD => DiskType::HDD,
                    _ => DiskType::Removable,
                }
            },
            mount_points: vec![mount_point],
            file_system: Some(disk.file_system().to_str().unwrap().to_string()),
            total_capacity: disk.total_space(),
            available_capacity: disk.available_space(),
            is_root_filesystem,
        });
    }))
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Volume>>();

    result.sort();

    return result;
}