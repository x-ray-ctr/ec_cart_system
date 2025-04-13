use crate::domain::product::entity::Product;
use crate::domain::product::repository::ProductRepository;
use crate::domain::error::DomainError;
use async_trait::async_trait;

/// **商品管理ユースケース**
pub struct ProductService<R: ProductRepository> {
    repo: R,
}

impl<R: ProductRepository> ProductService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// **商品一覧を取得**
    pub async fn list_products(&self) -> Result<Vec<Product>, DomainError> {
        self.repo.find_all().await
    }

    /// **Elasticsearch を利用した商品検索**
    pub async fn search_products(&self, query: &str) -> Result<Vec<Product>, DomainError> {
        // Elasticsearch との統合
        self.repo.find_all().await.map(|products| {
            products.into_iter()
                .filter(|p| p.name.contains(query) || p.description.contains(query))
                .collect()
        })
    }
}
