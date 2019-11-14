use crate::async_await;
use crate::domain::model;
use crate::domain::service;
use crate::error::ServiceError;
use crate::initializer;
use crate::server;
use futures::prelude::*;

pub struct WebContext {
    pub app: initializer::AppContext,
}

impl WebContext {
    pub fn new(config: initializer::Config) -> WebContext {
        WebContext {
            app: initializer::AppContext::new(config),
        }
    }

    /*
    fn auth_token(req: web::HttpRequest) -> Option<String> {
        req.headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(|v| v.to_owned())
    }

    async fn authorize(
        &self,
        req: web::HttpRequest,
        validate_user_role: Option<model::Role>,
    ) -> Result<model::AuthUser, error::Error> {
        let token = WebContext::auth_token(req)
            .ok_or(ServiceError::Unauthorized(failure::err_msg("Empty token")).to_http_error())?;
        let stoken = token.split("Bearer ").collect::<Vec<&str>>();
        if stoken.len() != 2 {
            return Err(
                ServiceError::Unauthorized(failure::err_msg("Invalid token")).to_http_error(),
            );
        }

        let user = self
            .app
            .services
            .auth_service
            .authorize(stoken[1])
            .await
            .map_err(|e| e.to_http_error())?;

        if let Some(check_role) = validate_user_role {
            if user.role[0] < check_role {
                return Err(
                    ServiceError::Unauthorized(failure::err_msg("Role is not enough"))
                        .to_http_error(),
                );
            }
        }

        Ok(user)
    }
    */
}

pub fn handlers(config: initializer::Config) -> server::App<WebContext> {
    server::App::new(WebContext::new(config))
        .route("/problems", hyper::Method::GET, api_problem_list)
        .route("/problems", hyper::Method::POST, api_problem_create)
}

async fn api_problem_list(
    _req: hyper::Request<hyper::Body>,
    _params: server::Params,
    data: std::sync::Arc<WebContext>,
) -> Result<hyper::Response<hyper::Body>, http::Error> {
    let problems = data
        .as_ref()
        .app
        .services
        .problem_service
        .list()
        .await
        .unwrap();

    let resp = hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .body(hyper::Body::from(serde_json::to_string(&problems).unwrap()))?;
    Ok(resp)
}

async fn api_problem_create(
    req: hyper::Request<hyper::Body>,
    _params: server::Params,
    data: std::sync::Arc<WebContext>,
) -> Result<hyper::Response<hyper::Body>, http::Error> {
    let body = String::from_utf8(req.into_body().try_concat().await.unwrap().to_vec()).unwrap();
    let req = serde_json::from_str::<service::ProblemCreateInput>(&body).unwrap();

    let problems = data
        .as_ref()
        .app
        .services
        .problem_service
        .create(req)
        .await
        .unwrap();

    let resp = hyper::Response::builder()
        .status(hyper::StatusCode::OK)
        .body(hyper::Body::from(serde_json::to_string(&problems).unwrap()))?;
    Ok(resp)
}
