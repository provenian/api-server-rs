use crate::async_await;
use crate::domain::model;
use crate::error::ServiceError;
use crate::initializer;
use actix_http::Response;
use actix_web::{error, web, HttpResponse};
use futures01::stream::Stream;

pub struct WebContext {
    pub app: initializer::AppContext,
}

impl WebContext {
    pub fn new(config: initializer::Config) -> WebContext {
        WebContext {
            app: initializer::AppContext::new(config),
        }
    }

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
}

pub fn handlers(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/echo").route(web::post().to_async(async_await::wrap3(api_echo))));
}

async fn api_echo(
    payload: web::Payload,
    context: web::Data<WebContext>,
    req: web::HttpRequest,
) -> Result<HttpResponse, error::Error> {
    let body = Box::new(
        futures::compat::Compat01As03::new(payload.concat2())
            .await
            .map_err(error::ErrorBadRequest)?,
    );
    let input = serde_json::from_slice::<String>(body.as_ref()).map_err(error::ErrorBadRequest)?;

    context
        .get_ref()
        .authorize(req, Some(model::Role::Writer))
        .await?;

    let res = context
        .app
        .services
        .echo_service
        .echo(input)
        .await
        .map_err(|e| e.to_http_error())?;

    Ok(Response::Ok().json(res))
}
