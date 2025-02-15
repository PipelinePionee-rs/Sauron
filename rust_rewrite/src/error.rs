use axum::response::{IntoResponse, Response};
use hyper::StatusCode;
use std::error::Error as StdError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  LoginFail,
  HashError(Box<dyn StdError + Send + Sync + 'static>),
}

impl IntoResponse for Error {
  fn into_response(self) -> Response {
    println!("->> {:<12} - {self:?}", "INTO_RES");

    (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED CLIENT_ERROR").into_response()
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