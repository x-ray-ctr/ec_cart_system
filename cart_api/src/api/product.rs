use axum::{extract::Query, Json, State};
use crate::usecase::product::service::ProductService;
use crate::domain::product::entity::Product;
use crate::domain::error::DomainError;
use std::sync::Arc;

/// **商品一覧を取得**
async fn list_products(
    State(service): State<Arc<ProductService<impl ProductRepository>>>,
) -> Result<Json<Vec<Product>>, DomainError> {
    let products = service.list_products().await?;
    Ok(Json(products))
}

/// **商品検索（Elasticsearch）**
async fn search_products(
    State(service): State<Arc<ProductService<impl ProductRepository>>>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<Vec<Product>>, DomainError> {
    let query = params.get("q").cloned().unwrap_or_default();
    let products = service.search_products(&query).await?;
    Ok(Json(products))
}
