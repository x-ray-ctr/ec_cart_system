#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Order {
    pub id: u64,
    pub user_id: u64,
    pub total_price: f64,
    pub created_at: chrono::NaiveDateTime,
}
