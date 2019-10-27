use crate::domain::interface::IJWTHandler;
use serde::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct JWTHandler {
    issuer: String,
}

impl JWTHandler {
    pub fn new(jwk_url: &str) -> JWTHandler {
        JWTHandler {
            issuer: "example.com".to_owned(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned + Clone> IJWTHandler<Payload> for JWTHandler {
    fn verify(&self, jwt: &str) -> Result<Payload, biscuit::errors::Error> {
        unimplemented!()
    }
}
