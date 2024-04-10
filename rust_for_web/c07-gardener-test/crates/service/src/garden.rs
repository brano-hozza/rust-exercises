use crate::error::ServiceError;
use async_trait::async_trait;
use model::garden::Garden;
use mongodb::Collection;

#[async_trait]
pub trait GardenService {
    async fn get(&self) -> Result<Garden, ServiceError>;
}

#[derive(Debug, Clone)]
pub struct GardenServiceImpl {
    pub collection: Collection<Garden>,
}

#[async_trait]
impl GardenService for GardenServiceImpl {
    async fn get(&self) -> Result<Garden, ServiceError> {
        let res = self.collection.find_one(None, None).await?;
        match res {
            Some(garden) => Ok(garden),
            None => Err(ServiceError::NotFound),
        }
    }
}
