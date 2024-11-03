use crate::aggregate::circle::Circle;
use anyhow::Error;

#[mockall::automock]
#[async_trait::async_trait]
pub trait CircleDuplicateCheckerInterface: Send + Sync {
    async fn check_circle_duplicate(&self, circle: &Circle) -> Result<(), Error>;
}
