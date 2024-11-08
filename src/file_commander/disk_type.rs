use serde::{Deserialize, Serialize};

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

