use crate::domain::model;
use crate::error::ServiceError;

pub struct EchoService {}

impl EchoService {
    pub fn new() -> EchoService {
        EchoService {}
    }

    pub async fn echo(&self, payload: String) -> Result<String, ServiceError> {
        Ok(payload)
    }
}

pub struct AuthService {}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {}
    }

    pub async fn authorize(&self, token: &str) -> Result<model::AuthUser, ServiceError> {
        unimplemented!()
    }
}
