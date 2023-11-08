use std::fmt::{Display, Formatter};
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginError,

    //Auth errors
    AuthFailNoAuthCookieFound,
    AuthFailTokenWrongFormat,

    //Model errors
    DeleteTicketIdNotFound {id: u64},
}

//So that custom error is displayed in a nice way
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RESP");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}