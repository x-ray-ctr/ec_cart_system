#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum OrderStatus {
    Pending,
    Paid,
    Shipped,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OrderStatusHistory {
    pub order_id: u64,
    pub status: OrderStatus,
    pub changed_at: chrono::NaiveDateTime,
}
