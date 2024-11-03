use crate::aggregate::{circle::Circle, value_object::circle_id::CircleId};
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleRepositoryInterface: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Circle>, Error>;
    async fn find_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    async fn create(&self, circle: &Circle) -> Result<(), Error>;
    async fn update(&self, circle: &Circle) -> Result<Circle, Error>;
    async fn delete(&self, circle: &Circle) -> Result<(), Error>;
}
