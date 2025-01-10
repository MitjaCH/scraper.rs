use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]  // Tells serde to use camelCase for JSON keys
pub struct Store {
    pub store_name: String,   // Will deserialize from "storeName"
    pub store_domain: String, // Will deserialize from "storeDomain"
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
