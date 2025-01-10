use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Store {
    pub store_name: String,
    pub store_domain: String,
}

#[derive(Serialize, Deserialize)]
pub struct Coupon {
    pub coupon_code: String,
    pub coupon_title: String,
    pub coupon_description: String,
    pub coupon_expiration_date: String,
}

#[derive(Serialize, Deserialize)]
pub struct StoreCoupons {
    pub store_name: String,
    pub store_domain: String,
    pub coupons: Vec<Coupon>,
}
