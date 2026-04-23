use serde_json::json;

use crate::error::{Error, Result};
use crate::http::SharedHttp;
use crate::types::{
    CityListResponse, DistrictByNameResponse, DistrictListResponse, ProvinceListResponse,
    SubDistrictListResponse,
};

#[derive(Debug, Clone)]
pub struct AddressService {
    pub(crate) client: SharedHttp,
}

impl AddressService {
    pub fn new(client: SharedHttp) -> Self {
        Self { client }
    }

    pub async fn provinces(&self) -> Result<ProvinceListResponse> {
        self.client.post_empty("/api/mitra/province").await
    }

    pub async fn cities(&self, provinsi_id: i64) -> Result<CityListResponse> {
        if provinsi_id <= 0 {
            return Err(Error::InvalidArgument(
                "provinsi_id must be greater than 0".into(),
            ));
        }
        self.client
            .post_json("/api/mitra/city", &json!({ "provinsi_id": provinsi_id }))
            .await
    }

    pub async fn districts(&self, kabupaten_id: i64) -> Result<DistrictListResponse> {
        if kabupaten_id <= 0 {
            return Err(Error::InvalidArgument(
                "kabupaten_id must be greater than 0".into(),
            ));
        }
        self.client
            .post_json(
                "/api/mitra/kecamatan",
                &json!({ "kabupaten_id": kabupaten_id }),
            )
            .await
    }

    pub async fn sub_districts(&self, kecamatan_id: i64) -> Result<SubDistrictListResponse> {
        if kecamatan_id <= 0 {
            return Err(Error::InvalidArgument(
                "kecamatan_id must be greater than 0".into(),
            ));
        }
        self.client
            .post_json(
                "/api/mitra/kelurahan",
                &json!({ "kecamatan_id": kecamatan_id }),
            )
            .await
    }

    pub async fn districts_by_name(&self, search: &str) -> Result<DistrictByNameResponse> {
        if search.is_empty() {
            return Err(Error::InvalidArgument("search must not be empty".into()));
        }
        self.client
            .post_json(
                "/api/mitra/v2/get_address_by_name",
                &json!({ "search": search }),
            )
            .await
    }
}
