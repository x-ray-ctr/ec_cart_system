use axum::{extract::Path, Json, State};
use crate::usecase::shipment::service::ShipmentService;
use crate::domain::shipment::entity::Shipment;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **発送情報を登録**
async fn ship_order(
    State(service): State<Arc<ShipmentService<impl ShipmentRepository>>>,
    Path(order_id): Path<u64>,
    Json(shipment): Json<Shipment>,
) -> Result<Json<()>, DomainError> {
    service.ship_order(order_id, shipment.tracking_number, shipment.address).await?;
    Ok(Json(()))
}
