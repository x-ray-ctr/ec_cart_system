use axum::{extract::Path, Json, State};
use crate::usecase::order::service::OrderService;
use crate::domain::order::entity::Order;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **カートを注文に変換**
async fn create_order(
    State(service): State<Arc<OrderService<impl OrderRepository, impl CartRepository>>>,
    Path(user_id): Path<u64>,
) -> Result<Json<Order>, DomainError> {
    let order = service.create_order(user_id).await?;
    Ok(Json(order))
}
