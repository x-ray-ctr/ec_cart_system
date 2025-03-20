#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum PaymentStatus {
    Pending,
    Completed,
    Failed,
    Refunded,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaymentStatusHistory {
    pub payment_id: u64,
    pub status: PaymentStatus,
    pub failed_reason: Option<String>,
    pub changed_at: chrono::NaiveDateTime,
}
