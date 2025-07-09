use axum::http::StatusCode;
use axum::response::Response;
use axum::response::IntoResponse;


pub type Result<T> = core::result::Result<T,Error>;

#[derive (Debug)]

pub enum Error{
    LoginFail,
} // Never ever pass you server error to the client is reveals a lot of things 

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR,"UNHANDLED_CLIENT_ERROR").into_response()

    }
}