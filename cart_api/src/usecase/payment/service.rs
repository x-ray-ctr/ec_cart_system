use crate::domain::payment::entity::Payment;
use crate::domain::payment::repository::{PaymentRepository, PaymentStatusRepository, PaymentGateway};
use crate::domain::payment::status::{PaymentStatus, PaymentStatusHistory};
use crate::domain::error::DomainError;
use chrono::Utc;

/// **決済のユースケース**
pub struct PaymentService<R: PaymentRepository, S: PaymentStatusRepository, G: PaymentGateway> {
    payment_repo: R,
    status_repo: S,
    gateway: G,
}

impl<R: PaymentRepository, S: PaymentStatusRepository, G: PaymentGateway> PaymentService<R, S, G> {
    pub fn new(payment_repo: R, status_repo: S, gateway: G) -> Self {
        Self {
            payment_repo,
            status_repo,
            gateway,
        }
    }

    /// **決済を処理**
    pub async fn process_payment(&self, payment: Payment) -> Result<(), DomainError> {
        // 1. Stripe で支払い処理
        let transaction_id = self.gateway.process_payment(&payment).await?;

        // 2. 決済成功した場合、DB に保存
        self.payment_repo.save(&payment).await?;
        self.status_repo.update_status(payment.id, PaymentStatus::Completed).await?;
        println!("Payment completed: Transaction ID: {}", transaction_id);

        Ok(())
    }
}
