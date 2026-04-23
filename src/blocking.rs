//! Synchronous (blocking) facade over the async [`crate::Client`].
//!
//! Enabled with the `blocking` cargo feature. Internally spins up a
//! single-threaded Tokio runtime and runs each call on it; from the caller's
//! perspective every method is plain synchronous.
//!
//! ```no_run
//! use kiriminaja::blocking::Client;
//! use kiriminaja::{Config, Env};
//!
//! # fn main() -> kiriminaja::Result<()> {
//! let client = Client::new(Config {
//!     env: Env::Sandbox,
//!     api_key: std::env::var("KIRIMINAJA_API_KEY").unwrap_or_default(),
//!     ..Default::default()
//! });
//!
//! let provinces = client.address.provinces()?;
//! println!("{:?}", provinces);
//! # Ok(()) }
//! ```
//!
//! ## Notes
//! - The internal runtime is shared between all services on a `Client`.
//! - Do not call blocking methods from inside an existing async runtime — use
//!   the async [`crate::Client`] there instead. Calling into a runtime from
//!   another runtime causes a panic from Tokio.

use std::sync::Arc;

use tokio::runtime::{Builder, Runtime};

use crate::config::Config;
use crate::error::Result;
use crate::services::address::AddressService as AsyncAddress;
use crate::services::courier::CourierService as AsyncCourier;
use crate::services::coverage_area::CoverageAreaService as AsyncCoverageArea;
use crate::services::credit::CreditService as AsyncCredit;
use crate::services::order::express::ExpressOrderService as AsyncExpress;
use crate::services::order::instant::InstantOrderService as AsyncInstant;
use crate::services::payment::PaymentService as AsyncPayment;
use crate::services::pickup::PickupService as AsyncPickup;
use crate::types::{
    CancelExpressOrderResponse, CancelInstantOrderResponse, CityListResponse,
    CourierDetailResponse, CourierGroupResponse, CourierListResponse, CreateInstantPickupResponse,
    CreditBalanceResponse, DistrictByNameResponse, DistrictListResponse, ExpressTrackingResponse,
    FindNewInstantDriverResponse, GetPaymentResponse, InstantPickupPayload, InstantTrackingResponse,
    KAResponse, PickupSchedulesResponse, PricingExpressPayload, PricingInstantPayload,
    ProvinceListResponse, RequestPickupPayload, SetCourierPreferenceResponse,
    SubDistrictListResponse,
};

type SharedRt = Arc<Runtime>;

fn new_runtime() -> SharedRt {
    Arc::new(
        Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to build tokio runtime for blocking client"),
    )
}

/// Synchronous version of [`crate::Client`].
#[derive(Clone)]
pub struct Client {
    pub address: AddressService,
    pub courier: CourierService,
    pub coverage_area: CoverageAreaService,
    pub credit: CreditService,
    pub order: OrderService,
    pub payment: PaymentService,
    pub pickup: PickupService,

    #[allow(dead_code)]
    rt: SharedRt,
}

impl Client {
    pub fn new(cfg: Config) -> Self {
        let inner = crate::Client::new(cfg);
        let rt = new_runtime();
        Self {
            address: AddressService { inner: inner.address.clone(), rt: rt.clone() },
            courier: CourierService { inner: inner.courier.clone(), rt: rt.clone() },
            coverage_area: CoverageAreaService {
                inner: inner.coverage_area.clone(),
                rt: rt.clone(),
            },
            credit: CreditService { inner: inner.credit.clone(), rt: rt.clone() },
            order: OrderService {
                express: ExpressOrderService {
                    inner: inner.order.express.clone(),
                    rt: rt.clone(),
                },
                instant: InstantOrderService {
                    inner: inner.order.instant.clone(),
                    rt: rt.clone(),
                },
            },
            payment: PaymentService { inner: inner.payment.clone(), rt: rt.clone() },
            pickup: PickupService { inner: inner.pickup.clone(), rt: rt.clone() },
            rt,
        }
    }
}

// ----- Address -----

#[derive(Clone)]
pub struct AddressService {
    inner: AsyncAddress,
    rt: SharedRt,
}

impl AddressService {
    pub fn provinces(&self) -> Result<ProvinceListResponse> {
        self.rt.block_on(self.inner.provinces())
    }
    pub fn cities(&self, provinsi_id: i64) -> Result<CityListResponse> {
        self.rt.block_on(self.inner.cities(provinsi_id))
    }
    pub fn districts(&self, kabupaten_id: i64) -> Result<DistrictListResponse> {
        self.rt.block_on(self.inner.districts(kabupaten_id))
    }
    pub fn sub_districts(&self, kecamatan_id: i64) -> Result<SubDistrictListResponse> {
        self.rt.block_on(self.inner.sub_districts(kecamatan_id))
    }
    pub fn districts_by_name(&self, search: &str) -> Result<DistrictByNameResponse> {
        self.rt.block_on(self.inner.districts_by_name(search))
    }
}

// ----- Courier -----

#[derive(Clone)]
pub struct CourierService {
    inner: AsyncCourier,
    rt: SharedRt,
}

impl CourierService {
    pub fn list(&self) -> Result<CourierListResponse> {
        self.rt.block_on(self.inner.list())
    }
    pub fn group(&self) -> Result<CourierGroupResponse> {
        self.rt.block_on(self.inner.group())
    }
    pub fn detail(&self, courier_code: &str) -> Result<CourierDetailResponse> {
        self.rt.block_on(self.inner.detail(courier_code))
    }
    pub fn set_whitelist_services(
        &self,
        services: &[String],
    ) -> Result<SetCourierPreferenceResponse> {
        self.rt.block_on(self.inner.set_whitelist_services(services))
    }
}

// ----- Coverage area -----

#[derive(Clone)]
pub struct CoverageAreaService {
    inner: AsyncCoverageArea,
    rt: SharedRt,
}

impl CoverageAreaService {
    pub fn pricing_express(&self, payload: &PricingExpressPayload) -> Result<KAResponse> {
        self.rt.block_on(self.inner.pricing_express(payload))
    }
    pub fn pricing_instant(&self, payload: &PricingInstantPayload) -> Result<KAResponse> {
        self.rt.block_on(self.inner.pricing_instant(payload))
    }
    pub fn provinces(&self) -> Result<ProvinceListResponse> {
        self.rt.block_on(self.inner.provinces())
    }
    pub fn cities(&self, provinsi_id: i64) -> Result<CityListResponse> {
        self.rt.block_on(self.inner.cities(provinsi_id))
    }
    pub fn districts(&self, kabupaten_id: i64) -> Result<DistrictListResponse> {
        self.rt.block_on(self.inner.districts(kabupaten_id))
    }
    pub fn sub_districts(&self, kecamatan_id: i64) -> Result<SubDistrictListResponse> {
        self.rt.block_on(self.inner.sub_districts(kecamatan_id))
    }
    pub fn districts_by_name(&self, search: &str) -> Result<DistrictByNameResponse> {
        self.rt.block_on(self.inner.districts_by_name(search))
    }
}

// ----- Credit -----

#[derive(Clone)]
pub struct CreditService {
    inner: AsyncCredit,
    rt: SharedRt,
}

impl CreditService {
    pub fn balance(&self) -> Result<CreditBalanceResponse> {
        self.rt.block_on(self.inner.balance())
    }
}

// ----- Order -----

#[derive(Clone)]
pub struct OrderService {
    pub express: ExpressOrderService,
    pub instant: InstantOrderService,
}

#[derive(Clone)]
pub struct ExpressOrderService {
    inner: AsyncExpress,
    rt: SharedRt,
}

impl ExpressOrderService {
    pub fn track(&self, order_id: &str) -> Result<ExpressTrackingResponse> {
        self.rt.block_on(self.inner.track(order_id))
    }
    pub fn cancel(&self, awb: &str, reason: &str) -> Result<CancelExpressOrderResponse> {
        self.rt.block_on(self.inner.cancel(awb, reason))
    }
    pub fn request_pickup(&self, payload: &RequestPickupPayload) -> Result<KAResponse> {
        self.rt.block_on(self.inner.request_pickup(payload))
    }
}

#[derive(Clone)]
pub struct InstantOrderService {
    inner: AsyncInstant,
    rt: SharedRt,
}

impl InstantOrderService {
    pub fn create(&self, payload: &InstantPickupPayload) -> Result<CreateInstantPickupResponse> {
        self.rt.block_on(self.inner.create(payload))
    }
    pub fn track(&self, order_id: &str) -> Result<InstantTrackingResponse> {
        self.rt.block_on(self.inner.track(order_id))
    }
    pub fn cancel(&self, order_id: &str) -> Result<CancelInstantOrderResponse> {
        self.rt.block_on(self.inner.cancel(order_id))
    }
    pub fn find_new_driver(&self, order_id: &str) -> Result<FindNewInstantDriverResponse> {
        self.rt.block_on(self.inner.find_new_driver(order_id))
    }
}

// ----- Payment -----

#[derive(Clone)]
pub struct PaymentService {
    inner: AsyncPayment,
    rt: SharedRt,
}

impl PaymentService {
    pub fn get_payment(&self, payment_id: &str) -> Result<GetPaymentResponse> {
        self.rt.block_on(self.inner.get_payment(payment_id))
    }
}

// ----- Pickup -----

#[derive(Clone)]
pub struct PickupService {
    inner: AsyncPickup,
    rt: SharedRt,
}

impl PickupService {
    pub fn schedules(&self) -> Result<PickupSchedulesResponse> {
        self.rt.block_on(self.inner.schedules())
    }
}
