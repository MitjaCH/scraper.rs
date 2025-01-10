mod scraper;
mod utils;
mod structs;
mod store;

use indicatif::{ProgressBar, ProgressStyle};
use crate::scraper::scrape_store;
use crate::utils::save_coupons;
use crate::store::load_stores;

#[tokio::main]
async fn main() {
    let stores = load_stores("C:/DEV/GBS/Projects/scraper/data/stores.json");
    let bar = ProgressBar::new(stores.len() as u64);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{bar} {pos}/{len}")
            .expect("Failed to create progress bar template")
            .progress_chars("=>-")
    );

    let mut all_coupons = vec![];

    for store in stores {
        let store_coupons = scrape_store(&store).await;
        all_coupons.push(store_coupons);
        bar.inc(1);
        save_coupons("./data/coupons.json", &all_coupons).await;
    }

    bar.finish_with_message("Scraping completed!");
}
