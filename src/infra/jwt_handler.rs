use crate::domain::interface::IJWTHandler;
use crate::error::ServiceError;
use serde::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct JWTHandler {
    issuer: String,
    public_key: Arc<biscuit::jwk::JWKSet<biscuit::Empty>>,
}

impl JWTHandler {
    pub fn new(jwk_url: &str) -> JWTHandler {
        JWTHandler {
            issuer: "example.com".to_owned(),
            public_key: Arc::new(JWTHandler::load_from_jwk(jwk_url)),
        }
    }

    fn load_from_jwk(jwk_url: &str) -> biscuit::jwk::JWKSet<biscuit::Empty> {
        reqwest::get(jwk_url).unwrap().json().unwrap()
    }

    fn get_key_from_jwk(&self, kid: &str) -> biscuit::jws::Secret {
        let key = self.public_key.as_ref().find(kid).unwrap();

        match key.algorithm {
            biscuit::jwk::AlgorithmParameters::RSA(ref params) => params.jws_public_key_secret(),
            _ => unimplemented!(),
        }
    }
}

impl<Payload: Serialize + serde::de::DeserializeOwned + Clone> IJWTHandler<Payload> for JWTHandler {
    fn verify(&self, jwt: &str) -> Result<Payload, ServiceError> {
        let bjwt: biscuit::JWT<Payload, biscuit::Empty> = biscuit::JWT::new_encoded(jwt);
        let kid = bjwt
            .unverified_header()
            .unwrap()
            .registered
            .key_id
            .clone()
            .unwrap();

        let bjwt = bjwt
            .into_decoded(
                &self.get_key_from_jwk(&kid),
                biscuit::jwa::SignatureAlgorithm::RS256,
            )
            .map_err(|err| {
                ServiceError::Unauthorized(failure::Error::from_boxed_compat(Box::new(err)))
            })?;
        bjwt.validate(std::default::Default::default())
            .map_err(|err| {
                ServiceError::Unauthorized(failure::Error::from_boxed_compat(Box::new(err)))
            })?;

        Ok(bjwt.payload().unwrap().private.clone())
    }
}
