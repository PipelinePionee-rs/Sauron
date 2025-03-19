use axum::{response::{IntoResponse, Response}, Json};
use hyper::StatusCode;
use std::error::Error as StdError;
use crate::models::ApiErrorResponse;

pub type Result<T> = core::result::Result<T, Error>;

// list of possible errors
#[derive(Debug)]
pub enum Error {
  LoginFail,
  InvalidCredentials,
  UsernameOrEmailExists,
  HashError(Box<dyn StdError + Send + Sync + 'static>),
  GenericError,
  UnprocessableEntity
}

// convert ApiErrorResponse to Response
impl IntoResponse for Error {
  fn into_response(self) -> Response {
    let (status, error, message) = match self {
      Error::LoginFail => (
        StatusCode::UNAUTHORIZED, 
        "Login failed", 
        "Login failed. please try again."
      ),
      Error::InvalidCredentials => (
        StatusCode::UNAUTHORIZED, 
        "Invalid credentials", 
        "Invalid credentials. please try again."
      ),
      Error::UnprocessableEntity => (
        StatusCode::UNPROCESSABLE_ENTITY, 
        "Unprocessable Entity", 
        "parameter 'q' cannot be empty or absent."
      ),
      Error::HashError(_) => (
        StatusCode::INTERNAL_SERVER_ERROR, 
        "Internal Server Error", 
        "Internal Server Error"
      ),
      Error::GenericError => (
        StatusCode::INTERNAL_SERVER_ERROR, 
        "Internal Server Error", 
        "Internal Server Error"
      ),
      Error::UsernameOrEmailExists => (
        StatusCode::CONFLICT,
        "Username or email is already in use",
        "Username or email is already in use. Please try a different email or username."
      ),
    };

    let body = Json(ApiErrorResponse {
      status_code: status.as_u16() as i32,
      error: error.to_string(),
      message: message.to_string(),
    });
    eprintln!("->> Error: status: {:?} {:?} ", body.status_code, body.error);
    (status, body).into_response()
  }
}

impl<E> From<E> for Error
where
    E: StdError + Send + Sync + 'static,
{
    fn from(err: E) -> Self {
        Error::HashError(Box::new(err))
    }
}