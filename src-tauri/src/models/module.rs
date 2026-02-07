use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ModuleInfo {
    pub name: String,
    pub description: String,
    pub path: String,
    pub location_count: usize,
    pub item_count: usize,
}
