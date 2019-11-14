use crate::domain::interface;
use crate::domain::model;
use crate::domain::service;
use crate::infra;
use crate::serviceclient;
use std::sync::Arc;

pub struct Config {
    pub database_url: String,
    pub jwk_url: String,
}

#[derive(Clone)]
pub struct Infras {
    pub db_pool: infra::conn_pool::ConnPool,
    pub jwt_handler: Arc<dyn interface::IJWTHandler<model::AuthUser> + Sync + Send>,
}

impl Infras {
    pub fn new(config: Config) -> Infras {
        Infras {
            db_pool: infra::conn_pool::ConnPool::from_url(&config.database_url),
            jwt_handler: Arc::new(infra::jwt_handler::JWTHandler::new(&config.jwk_url)),
        }
    }
}

#[derive(Clone)]
pub struct Serviceclients {
    pub problem_repo: Arc<serviceclient::ProblemRepository>,
}

impl Serviceclients {
    pub fn new(infras: Infras) -> Serviceclients {
        Serviceclients {
            problem_repo: Arc::new(serviceclient::ProblemRepository::new(infras.db_pool)),
        }
    }
}

#[derive(Clone)]
pub struct Services {
    pub auth_service: Arc<service::AuthService>,
    pub echo_service: Arc<service::EchoService>,
    pub problem_service: Arc<service::ProblemService>,
}

impl Services {
    pub fn new(infras: Infras, serviceclient: Serviceclients) -> Services {
        Services {
            auth_service: Arc::new(service::AuthService::new(infras.jwt_handler)),
            echo_service: Arc::new(service::EchoService::new()),
            problem_service: Arc::new(service::ProblemService::new(
                serviceclient.problem_repo.clone(),
            )),
        }
    }
}

pub struct AppContext {
    pub services: Services,
    pub serviceclient: Serviceclients,
    pub infras: Infras,
}

impl AppContext {
    pub fn new(config: Config) -> AppContext {
        let infras = Infras::new(config);
        let serviceclients = Serviceclients::new(infras.clone());
        let services = Services::new(infras.clone(), serviceclients.clone());

        AppContext {
            infras: infras,
            serviceclient: serviceclients,
            services: services,
        }
    }
}
