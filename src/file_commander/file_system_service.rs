use std::fmt::Debug;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sysinfo::System;
use log::error;

pub fn get_root(){

    let system = sysinfo::System::new_all();

    println!("{:?}",system);

    // If we don't have any physical core present, it's very likely that we're inside a VM...
    if system.physical_core_count().unwrap_or_default() > 0 {
        let mut disks = sysinfo::Disks::new();

        assert!(disks.list().is_empty());
        disks.refresh_list();

        for disk in disks.into_iter() {
            println!("{:?}",disk);

        }

        assert!(!disks.list().is_empty());
    }
    //fs::
}



#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum DiskType {
    SSD,
    HDD,
    Removable,
}

impl core::fmt::Display for DiskType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::SSD => "SSD",
            Self::HDD => "HDD",
            Self::Removable => "Removable",
        })
    }
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Volume {
    pub name: String,
    pub mount_points: Vec<PathBuf>,
    pub total_capacity: u64,
    pub available_capacity: u64,
    pub disk_type: DiskType,
    pub file_system: Option<String>,
    pub is_root_filesystem: bool,
}

impl Hash for Volume {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.mount_points.iter().for_each(|mount_point| {
            // Hashing like this to ignore ordering between mount points
            mount_point.hash(state);
        });
        self.disk_type.hash(state);
        self.file_system.hash(state);
    }
}

impl PartialEq for Volume {
    fn eq(&self, other: &Self) -> bool
    {
        self.name == other.name &&
        self.disk_type == other.disk_type &&
        self.file_system == other.file_system &&
        self.mount_points
            .iter()
            .all(|mount_point| other.mount_points.contains(mount_point))
    }
}

impl Eq for Volume {}


pub async fn get_volumes() -> Vec<Volume>
{
    let mut disks = sysinfo::Disks::new();
    disks.refresh_list();

    futures_util::future::join_all(disks.iter().map(|disk| async {
        #[cfg(not(windows))]
        let disk_name = disk.name();
        let mount_point: PathBuf = disk.mount_point().to_path_buf();

        #[cfg(windows)]
        let Ok((disk_name, mount_point)) = ({
            use normpath::PathExt;
            mount_point
                .normalize_virtually()
                .map(|p| (p.localize_name().to_os_string(), p.into_path_buf()))
        }) else {
            return None;
        };

        let is_root_filesystem = mount_point.is_absolute() && mount_point.parent().is_none();

        let mut name = disk_name.to_string_lossy().to_string();
        if name.replace(char::REPLACEMENT_CHARACTER, "") == "" {
            name = "Unknown".to_string()
        }

        Some(
            Volume {
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
            })
    }))
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Volume>>()
}