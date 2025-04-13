#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Payment {
    pub id: u64,
    pub order_id: u64,
    pub amount: f64,
    pub currency: String,
    pub payment_method: String,
    pub transaction_id: String,
}
