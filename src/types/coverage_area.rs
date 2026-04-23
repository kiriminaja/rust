use serde::{Deserialize, Serialize};

/// Express courier services accepted by the pricing endpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExpressService {
    Tiki,
    #[serde(rename = "posindonesia")]
    PosIndonesia,
    Paxel,
    Ninja,
    Rpx,
    #[serde(rename = "lion")]
    LionParcel,
    #[serde(rename = "jtcargo")]
    JTCargo,
    #[serde(rename = "sentral")]
    SentralCargo,
    #[serde(rename = "anteraja")]
    AnterAja,
    Ncs,
    Sicepat,
    Sap,
    #[serde(rename = "idx")]
    IdExpress,
    Jne,
    Jnt,
    Spx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstantService {
    GrabExpress,
    Borzo,
    Gosend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstantVehicle {
    #[serde(rename = "motor")]
    Bike,
    #[serde(rename = "mobil")]
    Car,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingExpressPayload {
    pub origin: i64,
    pub destination: i64,
    pub weight: i64,
    pub item_value: i64,
    pub insurance: i64,
    pub courier: Vec<ExpressService>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInstantLocationPayload {
    pub lat: f64,
    pub long: f64,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingInstantPayload {
    pub service: Vec<InstantService>,
    pub item_price: f64,
    pub origin: PricingInstantLocationPayload,
    pub destination: PricingInstantLocationPayload,
    pub weight: i64,
    pub vehicle: InstantVehicle,
    pub timezone: String,
}
