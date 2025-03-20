use stripe::{Client, Currency, PaymentIntent, CreatePaymentIntent};
use std::env;
use async_trait::async_trait;
use crate::domain::error::DomainError;
use crate::domain::payment::entity::Payment;
use crate::domain::payment::repository::PaymentGateway;

/// **Stripe API を利用する支払いゲートウェイ**
pub struct StripePaymentGateway {
    client: Client,
}

impl StripePaymentGateway {
    pub fn new() -> Self {
        let secret_key = env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        Self {
            client: Client::new(secret_key),
        }
    }
}

#[async_trait]
impl PaymentGateway for StripePaymentGateway {
    /// **Stripe で支払いを処理**
    async fn process_payment(&self, payment: &Payment) -> Result<String, DomainError> {
        let mut params = CreatePaymentIntent::new(payment.amount as i64, Currency::Usd);
        params.payment_method = Some(payment.payment_method.clone());

        let intent = PaymentIntent::create(&self.client, params).map_err(|e| {
            DomainError::PaymentError(format!("Stripe payment failed: {}", e))
        })?;

        Ok(intent.id)
    }
}
