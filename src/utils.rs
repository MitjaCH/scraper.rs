use crate::structs::StoreCoupons;
use serde_json::to_writer_pretty;
use std::fs::File;
use std::time::Duration;
use tokio::time::sleep;

pub async fn delay(ms: u64) {
    sleep(Duration::from_millis(ms)).await;
}

pub async fn save_coupons(file_path: &str, coupons: &[StoreCoupons]) {
    let file = File::create(file_path).unwrap();
    to_writer_pretty(file, &coupons).unwrap();
}