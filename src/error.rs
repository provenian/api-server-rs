#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "DB Error: {}", _0)]
    DBError(#[fail(cause)] debil_mysql::Error),

    #[fail(display = "DB Connector Error: {}", _0)]
    DBConnectorError(#[fail(cause)] crate::infra::conn_pool::DBConnectorError),

    #[fail(display = "Parse Error: {}", _0)]
    ParseError(#[fail(cause)] serde_json::Error),

    #[fail(display = "{}", _0)]
    GeneralError(failure::Error),

    #[fail(display = "Unauthorized: {}", _0)]
    Unauthorized(failure::Error),

    #[fail(display = "Invalid request: {}", _0)]
    InvalidRequest(Box<ServiceError>),

    #[fail(display = "Internal Server Error: {}", _0)]
    InternalServerError(Box<ServiceError>),
}

impl From<debil_mysql::Error> for ServiceError {
    fn from(err: debil_mysql::Error) -> ServiceError {
        ServiceError::DBError(err)
    }
}

impl From<crate::infra::conn_pool::DBConnectorError> for ServiceError {
    fn from(err: crate::infra::conn_pool::DBConnectorError) -> ServiceError {
        ServiceError::DBConnectorError(err)
    }
}

impl ServiceError {
    pub fn to_http_error(self) -> actix_web::Error {
        use ServiceError::*;

        match self {
            InvalidRequest(err) => actix_web::error::ErrorBadRequest(err),
            InternalServerError(err) => actix_web::error::ErrorInternalServerError(err),
            Unauthorized(err) => actix_web::error::ErrorUnauthorized(err),
            err => actix_web::error::ErrorInternalServerError(err),
        }
    }
}
