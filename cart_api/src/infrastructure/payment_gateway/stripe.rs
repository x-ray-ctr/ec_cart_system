use stripe::{Client, CreateCharge, Currency, ChargeSourceParams, Charge};
use std::env;
use async_trait::async_trait;
use crate::domain::payment::entity::Payment;
use crate::domain::payment::repository::PaymentGateway;
use crate::domain::error::DomainError;

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
    async fn process_payment(&self, payment: &Payment) -> Result<String, DomainError> {
        let mut params = CreateCharge::new();
        params.amount = Some((payment.amount * 100.0) as i64); // Stripe は最小単位（セントなど）で扱う
        params.currency = Some(Currency::Usd);
        params.source = Some(ChargeSourceParams::Token(payment.payment_method.clone()));

        let charge = Charge::create(&self.client, params).map_err(|e| {
            DomainError::PaymentError(format!("Stripe payment failed: {}", e))
        })?;

        Ok(charge.id)
    }
}
