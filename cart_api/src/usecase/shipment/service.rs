use crate::domain::shipment::entity::Shipment;
use crate::domain::shipment::repository::ShipmentRepository;
use crate::domain::error::DomainError;
use async_trait::async_trait;

/// **発送ユースケース**
pub struct ShipmentService<R: ShipmentRepository> {
    repo: R,
}

impl<R: ShipmentRepository> ShipmentService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// **注文の発送を登録**
    pub async fn ship_order(&self, order_id: u64, tracking_number: String, address: String) -> Result<Shipment, DomainError> {
        let shipment = Shipment {
            id: rand::random(),
            order_id,
            tracking_number,
            address,
            status: "Shipped".to_string(),
            shipped_at: Some(chrono::Utc::now().naive_utc()),
        };

        self.repo.save(&shipment).await?;
        Ok(shipment)
    }
}
