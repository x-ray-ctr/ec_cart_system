#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub id: u64,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: u32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
