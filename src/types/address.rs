use serde::{Deserialize, Serialize};

use super::BaseResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Province {
    pub id: i64,
    pub provinsi_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct City {
    pub id: i64,
    pub provinsi_id: i64,
    pub kabupaten_name: String,
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub postal_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct District {
    pub id: i64,
    pub kecamatan_name: String,
    pub kabupaten_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubDistrict {
    pub id: i64,
    pub kelurahan_name: String,
    pub kecamatan_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressByNameResult {
    pub id: i64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvinceListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<Province>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CityListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<City>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistrictListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub datas: Vec<District>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubDistrictListResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub results: Vec<SubDistrict>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistrictByNameResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub data: Vec<AddressByNameResult>,
}
