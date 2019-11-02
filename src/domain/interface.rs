use crate::domain::*;
use crate::error::ServiceError;
use async_trait::async_trait;

pub trait IJWTHandler<Payload> {
    fn verify(&self, jwt: &str) -> Result<Payload, ServiceError>;
}

#[async_trait]
pub trait IProblemRepository {
    async fn save(&self, problem: model::Problem) -> Result<(), ServiceError>;
    async fn list(&self) -> Result<Vec<model::ProblemSummary>, ServiceError>;
    async fn list_by_tag(&self, tag: String) -> Result<Vec<model::ProblemSummary>, ServiceError>;
    async fn find_by_id(&self, key: String) -> Result<Option<model::Problem>, ServiceError>;
}
