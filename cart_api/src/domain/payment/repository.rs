use async_trait::async_trait;
use crate::domain::payment::entity::Payment;
use crate::domain::error::DomainError;

/// **決済データを扱うリポジトリ**
#[async_trait]
pub trait PaymentRepository {
    async fn find(&self, id: u64) -> Result<Payment, DomainError>;
    async fn save(&self, payment: &Payment) -> Result<(), DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
}
