use crate::domain::cart::entity::{Cart, CartItem};
use crate::domain::cart::repository::CartRepository;
use crate::domain::error::DomainError;
use async_trait::async_trait;

/// **カート管理ユースケース**
pub struct CartService<R: CartRepository> {
    repo: R,
}

impl<R: CartRepository> CartService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// **カートに商品を追加**
    pub async fn add_to_cart(&self, user_id: u64, item: CartItem) -> Result<Cart, DomainError> {
        let mut cart = self.repo.find(user_id).await.unwrap_or_else(|_| Cart {
            user_id,
            items: vec![],
            created_at: chrono::Utc::now().naive_utc(),
        });

        cart.items.push(item);
        self.repo.save(&cart).await?;
        Ok(cart)
    }

    /// **カートをクリア**
    pub async fn clear_cart(&self, user_id: u64) -> Result<(), DomainError> {
        self.repo.delete(user_id).await
    }
}
