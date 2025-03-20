use crate::domain::payment::entity::Payment;
use crate::domain::payment::repository::{PaymentRepository, PaymentGateway};
use crate::domain::error::DomainError;
use async_trait::async_trait;

/// **決済ユースケース**
pub struct PaymentService<R: PaymentRepository, G: PaymentGateway> {
    payment_repo: R,
    gateway: G,
}

impl<R: PaymentRepository, G: PaymentGateway> PaymentService<R, G> {
    pub fn new(payment_repo: R, gateway: G) -> Self {
        Self { payment_repo, gateway }
    }

    /// **決済を処理**
    pub async fn process_payment(&self, payment: Payment) -> Result<String, DomainError> {
        let transaction_id = self.gateway.process_payment(&payment).await?;
        self.payment_repo.save(&payment).await?;
        Ok(transaction_id)
    }
}
