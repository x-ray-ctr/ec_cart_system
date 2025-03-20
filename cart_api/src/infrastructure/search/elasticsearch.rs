use elasticsearch::{Elasticsearch, SearchParts};
use serde_json::json;
use async_trait::async_trait;
use crate::domain::product::{entity::Product, repository::ProductRepository};
use crate::domain::error::DomainError;

/// **Elasticsearch の商品リポジトリ**
pub struct ElasticsearchProductRepository {
    client: Elasticsearch,
}

impl ElasticsearchProductRepository {
    pub fn new(client: Elasticsearch) -> Self {
        Self { client }
    }
}

#[async_trait]
impl ProductRepository for ElasticsearchProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, DomainError> {
        let response = self.client
            .search(SearchParts::Index(&["products"]))
            .body(json!({
                "query": { "match_all": {} }
            }))
            .send()
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        let body = response.json::<serde_json::Value>().await.map_err(|e| DomainError::RepositoryError(e.to_string()))?;
        let hits = body["hits"]["hits"].as_array().unwrap_or(&vec![]);
        
        let products = hits.iter().filter_map(|hit| serde_json::from_value(hit["_source"].clone()).ok()).collect();
        
        Ok(products)
    }
}
