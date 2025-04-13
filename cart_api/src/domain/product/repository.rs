use async_trait::async_trait;
use crate::domain::product::entity::Product;
use crate::domain::error::DomainError;

/// **商品データを扱うリポジトリ**
#[async_trait]
pub trait ProductRepository {
    async fn find(&self, id: u64) -> Result<Product, DomainError>;
    async fn save(&self, product: &Product) -> Result<(), DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
    async fn find_all(&self) -> Result<Vec<Product>, DomainError>;
}
