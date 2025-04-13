#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CartItem {
    pub product_id: u64,
    pub quantity: u32,
    pub price_at_time: f64, // 購入時の価格を保存
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cart {
    pub user_id: u64,
    pub items: Vec<CartItem>,
    pub created_at: chrono::NaiveDateTime,
}
