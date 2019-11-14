#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "DB Error: {}", _0)]
    DBError(#[fail(cause)] debil_mysql::Error),

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
