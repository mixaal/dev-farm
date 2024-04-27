use salvo::http::ParseError;
use salvo::{http::StatusCode, Response};
use serde::Serialize;

use crate::rest::AppError;

pub fn parse_error(res: &mut Response, err: ParseError) {
    let app_err = AppError {
        error: err.to_string(),
    };
    http_response(res, StatusCode::BAD_REQUEST, &app_err);
}

pub fn io_error(res: &mut Response, err: std::io::Error) {
    let app_err = AppError {
        error: err.to_string(),
    };
    http_response(res, StatusCode::BAD_REQUEST, &app_err);
}

pub fn http_response<T: Serialize>(res: &mut Response, status: StatusCode, val: &T) {
    res.status_code(status);
    let r = serde_json::to_string(val);
    if r.is_err() {
        res.write_body("{\"error\": \"can't serialize json response\"}")
            .ok();
    } else {
        res.write_body(r.unwrap()).ok();
    }
}
