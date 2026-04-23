use serde::{Deserialize, Serialize};

use super::BaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickupScheduleItem {
    #[serde(default)]
    pub clock: String,
    #[serde(default)]
    pub until: String,
    #[serde(default)]
    pub expired: i64,
    #[serde(default)]
    pub libur: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickupSchedulesResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub schedules: Vec<PickupScheduleItem>,
}
