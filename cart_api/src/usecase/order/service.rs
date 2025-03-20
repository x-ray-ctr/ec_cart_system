use crate::domain::order::entity::Order;
use crate::domain::order::repository::OrderRepository;
use crate::domain::cart::repository::CartRepository;
use crate::domain::error::DomainError;
use async_trait::async_trait;

/// **注文管理ユースケース**
pub struct OrderService<R: OrderRepository, C: CartRepository> {
    order_repo: R,
    cart_repo: C,
}

impl<R: OrderRepository, C: CartRepository> OrderService<R, C> {
    pub fn new(order_repo: R, cart_repo: C) -> Self {
        Self { order_repo, cart_repo }
    }

    /// **カートから注文を作成**
    pub async fn create_order(&self, user_id: u64) -> Result<Order, DomainError> {
        let cart = self.cart_repo.find(user_id).await?;
        let total_price: f64 = cart.items.iter().map(|item| item.price_at_time * item.quantity as f64).sum();

        let order = Order {
            id: rand::random(),
            user_id,
            total_price,
            created_at: chrono::Utc::now().naive_utc(),
        };

        self.order_repo.save(&order).await?;
        self.cart_repo.delete(user_id).await?;
        Ok(order)
    }
}
