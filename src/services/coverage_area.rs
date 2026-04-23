use crate::error::Result;
use crate::http::SharedHttp;
use crate::services::address::AddressService;
use crate::types::{
    CityListResponse, DistrictByNameResponse, DistrictListResponse, KAResponse,
    PricingExpressPayload, PricingInstantPayload, ProvinceListResponse, SubDistrictListResponse,
};

#[derive(Debug, Clone)]
pub struct CoverageAreaService {
    addr: AddressService,
    client: SharedHttp,
}

impl CoverageAreaService {
    pub fn new(client: SharedHttp) -> Self {
        Self {
            addr: AddressService::new(client.clone()),
            client,
        }
    }

    pub async fn pricing_express(&self, payload: &PricingExpressPayload) -> Result<KAResponse> {
        self.client
            .post_json("/api/mitra/v6.1/shipping_price", payload)
            .await
    }

    pub async fn pricing_instant(&self, payload: &PricingInstantPayload) -> Result<KAResponse> {
        self.client
            .post_json("/api/mitra/v4/instant/pricing", payload)
            .await
    }

    pub async fn provinces(&self) -> Result<ProvinceListResponse> {
        self.addr.provinces().await
    }

    pub async fn cities(&self, provinsi_id: i64) -> Result<CityListResponse> {
        self.addr.cities(provinsi_id).await
    }

    pub async fn districts(&self, kabupaten_id: i64) -> Result<DistrictListResponse> {
        self.addr.districts(kabupaten_id).await
    }

    pub async fn sub_districts(&self, kecamatan_id: i64) -> Result<SubDistrictListResponse> {
        self.addr.sub_districts(kecamatan_id).await
    }

    pub async fn districts_by_name(&self, search: &str) -> Result<DistrictByNameResponse> {
        self.addr.districts_by_name(search).await
    }
}
