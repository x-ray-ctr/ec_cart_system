use axum::{extract::Path, Json, State};
use crate::usecase::payment::service::PaymentService;
use crate::domain::payment::entity::Payment;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **決済を処理する API**
async fn process_payment(
    State(service): State<Arc<PaymentService>>,
    Json(payment): Json<Payment>,
) -> Result<Json<()>, DomainError> {
    service.process_payment(payment).await?;
    Ok(Json(()))
}
