use crate::error::ServiceError;

pub trait IJWTHandler<Payload> {
    fn verify(&self, jwt: &str) -> Result<Payload, ServiceError>;
}
