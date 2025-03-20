use async_trait::async_trait;
use crate::domain::payment::entity::Payment;
use crate::domain::error::DomainError;

/// **決済プロバイダーの共通インターフェース**
#[async_trait]
pub trait PaymentGateway {
    async fn process_payment(&self, payment: &Payment) -> Result<String, DomainError>;
}
