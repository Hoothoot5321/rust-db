use std::fmt;

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[derive(Debug)]
pub struct CustErr {
    pub msg: String,
    pub code: u16,
}

impl fmt::Display for CustErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.msg)
    }
}

impl actix_web::error::ResponseError for CustErr {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.msg.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::from_u16(self.code).unwrap()
    }
}

pub fn create_err(msg: String, code: u16) -> CustErr {
    CustErr { msg, code }
}
