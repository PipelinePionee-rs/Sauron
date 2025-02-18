use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::error::Error as StdError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  LoginFail,
  InvalidCredentials,
  HashError(Box<dyn StdError + Send + Sync + 'static>),
  GenericError
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> {:<12} - {self:?}", "INTO_RES");
    match self {
      Error::LoginFail => {
        (StatusCode::UNAUTHORIZED, "Login failed").into_response()
      }
      Error::InvalidCredentials => {
        (StatusCode::UNAUTHORIZED, "Invalid credentials").into_response()
      }
      Error::HashError(_) => {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
      }
      Error::GenericError => {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
      }
    }
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