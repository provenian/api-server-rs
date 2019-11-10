use crate::domain::interface::*;
use crate::domain::model::*;
use crate::error::ServiceError;
use serde::*;
use std::sync::Arc;

pub struct EchoService {}

impl EchoService {
    pub fn new() -> EchoService {
        EchoService {}
    }

    pub async fn echo(&self, payload: String) -> Result<String, ServiceError> {
        Ok(payload)
    }
}

pub struct AuthService {
    jwt_handler: Arc<dyn IJWTHandler<AuthUser> + Sync + Send>,
}

impl AuthService {
    pub fn new(jwt_handler: Arc<dyn IJWTHandler<AuthUser> + Sync + Send>) -> AuthService {
        AuthService {
            jwt_handler: jwt_handler,
        }
    }

    pub async fn authorize(&self, token: &str) -> Result<AuthUser, ServiceError> {
        self.jwt_handler.as_ref().verify(token)
    }
}

pub struct ProblemService {
    problem_repo: Arc<dyn IProblemRepository + Sync + Send>,
}

#[derive(Deserialize)]
pub struct ProblemCreateInput {
    title: String,
    content: String,
    writer: String,
}

impl ProblemService {
    pub fn new(problem_repo: Arc<dyn IProblemRepository + Sync + Send>) -> ProblemService {
        ProblemService {
            problem_repo: problem_repo,
        }
    }

    pub async fn list(&self) -> Result<Vec<ProblemSummary>, ServiceError> {
        self.problem_repo.list().await
    }

    pub async fn create(&self, input: ProblemCreateInput) -> Result<(), ServiceError> {
        self.problem_repo
            .save(Problem::new(input.title, input.content, input.writer))
            .await
    }
}
