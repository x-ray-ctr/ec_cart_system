use async_trait::async_trait;
use crate::domain::order::entity::Order;
use crate::domain::error::DomainError;

/// **注文データを扱うリポジトリ**
#[async_trait]
pub trait OrderRepository {
    async fn find(&self, id: u64) -> Result<Order, DomainError>;
    async fn save(&self, order: &Order) -> Result<(), DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
}
