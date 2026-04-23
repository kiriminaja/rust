use serde::{Deserialize, Serialize};

use super::{BaseResponse, InstantService, InstantVehicle, KAResponse};

// ----- Express request payloads -----

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPickupItemMetadata {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub sku: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub variant_label: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPickupItem {
    pub name: String,
    pub price: i64,
    pub qty: i64,
    pub weight: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub width: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub length: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub height: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<RequestPickupItemMetadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPickupPackage {
    pub order_id: String,
    pub destination_name: String,
    pub destination_phone: String,
    pub destination_address: String,
    pub destination_kecamatan_id: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub destination_kelurahan_id: i64,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub destination_zipcode: String,
    pub weight: i64,
    pub width: i64,
    pub length: i64,
    pub height: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub qty: i64,
    pub item_value: i64,
    pub shipping_cost: i64,
    pub service: String,
    pub service_type: String,
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub insurance_amount: f64,
    pub cod: i64,
    pub package_type_id: i64,
    pub item_name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RequestPickupItem>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub drop: bool,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub note: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestPickupPayload {
    pub address: String,
    pub phone: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub zipcode: String,
    pub kecamatan_id: i64,
    #[serde(default, skip_serializing_if = "is_zero_i64")]
    pub kelurahan_id: i64,
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub latitude: f64,
    #[serde(default, skip_serializing_if = "is_zero_f64")]
    pub longitude: f64,
    pub packages: Vec<RequestPickupPackage>,
    pub schedule: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub platform_name: String,
}

// ----- Instant request payloads -----

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantPickupItem {
    pub name: String,
    pub description: String,
    pub price: i64,
    pub weight: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantPickupPackage {
    pub origin_name: String,
    pub origin_phone: String,
    pub origin_lat: f64,
    pub origin_long: f64,
    pub origin_address: String,
    pub origin_address_note: String,
    pub destination_name: String,
    pub destination_phone: String,
    pub destination_lat: f64,
    pub destination_long: f64,
    pub destination_address: String,
    pub destination_address_note: String,
    pub shipping_price: i64,
    pub item: InstantPickupItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantPickupPayload {
    pub service: InstantService,
    pub service_type: String,
    pub vehicle: InstantVehicle,
    pub order_prefix: String,
    pub packages: Vec<InstantPickupPackage>,
}

// ----- Express tracking responses -----

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpressTrackingImages {
    pub camera_img: Option<String>,
    pub signature_img: Option<String>,
    pub pop_img: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpressTrackingCosts {
    #[serde(default)]
    pub add_cost: f64,
    #[serde(default)]
    pub currency: String,
    #[serde(default)]
    pub cod: f64,
    #[serde(default)]
    pub insurance_amount: f64,
    #[serde(default)]
    pub insurance_percent: f64,
    #[serde(default)]
    pub discount_amount: f64,
    #[serde(default)]
    pub subsidi_amount: f64,
    #[serde(default)]
    pub shipping_cost: f64,
    #[serde(default)]
    pub correction: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpressTrackingLocation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub city: String,
    #[serde(default)]
    pub zip_code: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExpressTrackingDetails {
    pub awb: Option<String>,
    pub signature_code: Option<String>,
    pub sorting_code: Option<String>,
    #[serde(default)]
    pub order_id: String,
    pub status_code: Option<i64>,
    #[serde(default)]
    pub estimation: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub service_name: String,
    #[serde(default)]
    pub drop: bool,
    pub shipped_at: Option<String>,
    #[serde(default)]
    pub delivered: bool,
    pub delivered_at: Option<String>,
    #[serde(default)]
    pub refunded: bool,
    pub refunded_at: Option<String>,
    #[serde(default)]
    pub images: ExpressTrackingImages,
    #[serde(default)]
    pub costs: ExpressTrackingCosts,
    #[serde(default)]
    pub origin: ExpressTrackingLocation,
    #[serde(default)]
    pub destination: ExpressTrackingLocation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressTrackingHistory {
    pub created_at: String,
    pub status: String,
    pub status_code: i64,
    #[serde(default)]
    pub driver: String,
    #[serde(default)]
    pub receiver: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpressTrackingResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub details: ExpressTrackingDetails,
    #[serde(default)]
    pub histories: Vec<ExpressTrackingHistory>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelExpressOrderData {
    #[serde(default)]
    pub success: String,
    #[serde(default)]
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelExpressOrderResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub data: CancelExpressOrderData,
}

// ----- Instant tracking responses -----

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstantTrackingDriver {
    pub name: Option<String>,
    pub phone: Option<String>,
    pub photo: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstantTrackingLocation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub address_note: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub lat: f64,
    #[serde(default)]
    pub long: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstantTrackingDate {
    #[serde(default)]
    pub created_at: String,
    pub finished_at: Option<String>,
    pub allocated_at: Option<String>,
    pub canceled_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstantTrackingCost {
    #[serde(default)]
    pub shipping_cost: f64,
    pub insurance: Option<f64>,
    #[serde(default)]
    pub admin_fee: f64,
    #[serde(default)]
    pub total_price: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InstantTrackingItem {
    #[serde(default)]
    pub price: f64,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantTrackingResult {
    #[serde(default)]
    pub driver: InstantTrackingDriver,
    #[serde(default)]
    pub origin: InstantTrackingLocation,
    #[serde(default)]
    pub destination: InstantTrackingLocation,
    #[serde(default)]
    pub date: InstantTrackingDate,
    #[serde(default)]
    pub cost: InstantTrackingCost,
    #[serde(default)]
    pub item: InstantTrackingItem,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub service_type: String,
    #[serde(default)]
    pub tracking_code: String,
    pub cancel_description: Option<String>,
    pub live_tracking_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstantTrackingResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub result: Option<InstantTrackingResult>,
}

// ----- Cancel instant -----

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelInstantPayment {
    #[serde(default)]
    pub payment_id: String,
    #[serde(default)]
    pub amount: f64,
    #[serde(default)]
    pub status_code: i64,
    pub qr_content: Option<String>,
    #[serde(default)]
    pub pay_time: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelInstantPackageLocation {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub address: String,
    #[serde(default)]
    pub phone: String,
    #[serde(default)]
    pub latitude: f64,
    #[serde(default)]
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelInstantPackage {
    #[serde(default)]
    pub awb: String,
    #[serde(default)]
    pub order_id: String,
    #[serde(default)]
    pub service: String,
    #[serde(default)]
    pub service_type: String,
    #[serde(default)]
    pub status: i64,
    pub live_track_url: Option<String>,
    #[serde(default)]
    pub polyline: String,
    #[serde(default)]
    pub origin: CancelInstantPackageLocation,
    #[serde(default)]
    pub destination: CancelInstantPackageLocation,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CancelInstantOrderResult {
    #[serde(default)]
    pub payment: CancelInstantPayment,
    #[serde(default)]
    pub packages: Vec<CancelInstantPackage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelInstantOrderResponse {
    #[serde(flatten)]
    pub base: BaseResponse,
    #[serde(default)]
    pub result: CancelInstantOrderResult,
}

pub type CreateInstantPickupResponse = KAResponse;
pub type FindNewInstantDriverResponse = KAResponse;

// ----- Helpers used in `skip_serializing_if` -----

fn is_zero_i64(v: &i64) -> bool {
    *v == 0
}

fn is_zero_f64(v: &f64) -> bool {
    *v == 0.0
}

fn is_false(v: &bool) -> bool {
    !*v
}
