use crate::structs::Store;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;

pub fn load_stores(file_path: &str) -> Vec<Store> {
    let file = File::open(file_path).expect("Failed to open stores.json");
    let reader = BufReader::new(file);
    from_reader(reader).expect("Invalid JSON format")
}
