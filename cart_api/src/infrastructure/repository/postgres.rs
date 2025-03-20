use sqlx::{PgPool, Error};
use async_trait::async_trait;
use crate::domain::product::{entity::Product, repository::ProductRepository};
use crate::domain::error::DomainError;

/// **PostgreSQL の商品リポジトリ**
pub struct PostgresProductRepository {
    pool: PgPool,
}

impl PostgresProductRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProductRepository for PostgresProductRepository {
    async fn find(&self, id: u64) -> Result<Product, DomainError> {
        sqlx::query_as!(Product, "SELECT * FROM products WHERE id = $1", id as i64)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    async fn find_all(&self) -> Result<Vec<Product>, DomainError> {
        sqlx::query_as!(Product, "SELECT * FROM products")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))
    }

    async fn save(&self, product: &Product) -> Result<(), DomainError> {
        sqlx::query!(
            "INSERT INTO products (id, name, description, price, stock, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            product.id as i64,
            product.name,
            product.description,
            product.price,
            product.stock as i32,
            product.created_at,
            product.updated_at,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: u64) -> Result<(), DomainError> {
        sqlx::query!("DELETE FROM products WHERE id = $1", id as i64)
            .execute(&self.pool)
            .await
            .map_err(|e| DomainError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}
