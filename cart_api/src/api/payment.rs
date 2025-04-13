use axum::{extract::Path, Json, State};
use crate::usecase::payment::service::PaymentService;
use crate::domain::payment::entity::Payment;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **決済を処理**
async fn process_payment(
    State(service): State<Arc<PaymentService<impl PaymentRepository, impl PaymentGateway>>>,
    Json(payment): Json<Payment>,
) -> Result<Json<String>, DomainError> {
    let transaction_id = service.process_payment(payment).await?;
    Ok(Json(transaction_id))
}
