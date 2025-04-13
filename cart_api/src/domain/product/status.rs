#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ProductStatus {
    Available,
    OutOfStock,
    Discontinued,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProductStatusHistory {
    pub product_id: u64,
    pub status: ProductStatus,
    pub changed_at: chrono::NaiveDateTime,
}
