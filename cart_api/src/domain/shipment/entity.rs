#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Shipment {
    pub id: u64,
    pub order_id: u64,
    pub tracking_number: String,
    pub address: String,
    pub status: String,
    pub shipped_at: Option<chrono::NaiveDateTime>,
}
