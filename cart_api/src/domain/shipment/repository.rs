use async_trait::async_trait;
use crate::domain::shipment::entity::Shipment;
use crate::domain::error::DomainError;

/// **発送データを扱うリポジトリ**
#[async_trait]
pub trait ShipmentRepository {
    async fn find(&self, id: u64) -> Result<Shipment, DomainError>;
    async fn save(&self, shipment: &Shipment) -> Result<(), DomainError>;
    async fn delete(&self, id: u64) -> Result<(), DomainError>;
}
