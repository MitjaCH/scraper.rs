use crate::structs::{Coupon, Store, StoreCoupons};
use crate::utils::delay;
use regex::Regex;
use reqwest::Client;
use serde_json::json;

pub async fn fetch_coupon_ids(client: &Client, domain: &str) -> Vec<String> {
    let url = format!("https://couponfollow.com/site/{}", domain);
    let response = client.get(&url).send().await.unwrap().text().await.unwrap();
    let regex = Regex::new(r#"data-cid="(\d+)""#).unwrap();
    regex.captures_iter(&response).filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string())).collect()
}

pub async fn fetch_coupon_data(client: &Client, coupon_id: &str, domain: &str) -> Option<Coupon> {
    let url = "https://couponfollow.com/portalapi/coupon/popup";
    let body = json!({ "id": coupon_id, "domainName": domain });
    
    for _ in 0..3 {
        if let Ok(response) = client.post(url).json(&body).send().await {
            if let Ok(data) = response.json::<serde_json::Value>().await {
                return Some(Coupon {
                    coupon_code: data["code"].as_str().unwrap_or("").to_string(),
                    coupon_title: data["title"].as_str().unwrap_or("").to_string(),
                    coupon_description: data["desc"].as_str().unwrap_or("").to_string(),
                    coupon_expiration_date: data["expirationDate"].as_str().unwrap_or("").to_string(),
                });
            }
        }
        delay(3000).await;
    }
    None
}

pub async fn scrape_store(store: &Store) -> StoreCoupons {
    let client = Client::new();
    let coupon_ids = fetch_coupon_ids(&client, &store.store_domain).await;
    let mut coupons = vec![];

    for coupon_id in coupon_ids {
        if let Some(coupon) = fetch_coupon_data(&client, &coupon_id, &store.store_domain).await {
            coupons.push(coupon);
        }
    }

    StoreCoupons {
        store_name: store.store_name.clone(),
        store_domain: store.store_domain.clone(),
        coupons,
    }
}
