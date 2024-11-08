use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use crate::file_commander::disk_type::DiskType;

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

impl Volume {
    pub fn get_drive_name(&self) -> String {
        if self.mount_points.is_empty() {
            return "Unknown".to_owned();
        }

        self.mount_points[0].to_str().unwrap().to_string()
    }
}

impl PartialOrd for Volume {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Volume::cmp(self, other))
    }
}

impl Ord for Volume {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl Hash for Volume {
    fn hash<H: Hasher>(&self, state: &mut H)
    {
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
