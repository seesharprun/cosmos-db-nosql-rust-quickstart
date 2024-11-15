use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: String,
    pub category: String,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
    pub clearance: bool,
}