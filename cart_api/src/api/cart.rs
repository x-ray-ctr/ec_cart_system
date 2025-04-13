use axum::{extract::Path, Json, State};
use crate::usecase::cart::service::CartService;
use crate::domain::cart::entity::CartItem;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **カートに商品を追加**
async fn add_to_cart(
    State(service): State<Arc<CartService<impl CartRepository>>>,
    Path(user_id): Path<u64>,
    Json(item): Json<CartItem>,
) -> Result<Json<()>, DomainError> {
    service.add_to_cart(user_id, item).await?;
    Ok(Json(()))
}
