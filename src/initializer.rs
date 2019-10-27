use crate::domain::interface;
use crate::domain::model;
use crate::domain::service;
use crate::infra;
use std::sync::Arc;

pub struct Config {
    pub database_url: String,
    pub jwk_url: String,
}

pub struct Infras {
    pub jwt_handler: Arc<dyn interface::IJWTHandler<model::AuthUser> + Sync + Send>,
}

impl Infras {
    pub fn new(config: Config) -> Infras {
        Infras {
            jwt_handler: Arc::new(infra::jwt_handler::JWTHandler::new(&config.jwk_url)),
        }
    }
}

pub struct Services {
    pub auth_service: Arc<service::AuthService>,
    pub echo_service: Arc<service::EchoService>,
}

impl Services {
    pub fn new() -> Services {
        Services {
            auth_service: Arc::new(service::AuthService::new()),
            echo_service: Arc::new(service::EchoService::new()),
        }
    }
}

pub struct AppContext {
    pub services: Services,
    pub infras: Infras,
}

impl AppContext {
    pub fn new(config: Config) -> AppContext {
        AppContext {
            services: Services::new(),
            infras: Infras::new(config),
        }
    }
}
