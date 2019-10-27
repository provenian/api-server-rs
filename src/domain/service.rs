use crate::domain::interface;
use crate::domain::model;
use crate::error::ServiceError;
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
    jwt_handler: Arc<dyn interface::IJWTHandler<model::AuthUser> + Sync + Send>,
}

impl AuthService {
    pub fn new(
        jwt_handler: Arc<dyn interface::IJWTHandler<model::AuthUser> + Sync + Send>,
    ) -> AuthService {
        AuthService {
            jwt_handler: jwt_handler,
        }
    }

    pub async fn authorize(&self, token: &str) -> Result<model::AuthUser, ServiceError> {
        self.jwt_handler.as_ref().verify(token)
    }
}
