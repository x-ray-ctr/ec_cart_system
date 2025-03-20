use async_trait::async_trait;
use crate::domain::cart::entity::Cart;
use crate::domain::error::DomainError;

/// **カートのリポジトリ**
#[async_trait]
pub trait CartRepository {
    async fn find(&self, user_id: u64) -> Result<Cart, DomainError>;
    async fn save(&self, cart: &Cart) -> Result<(), DomainError>;
    async fn delete(&self, user_id: u64) -> Result<(), DomainError>;
}
