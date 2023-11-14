use actix_web::{get, Error, HttpResponse, Responder};

use crate::frontend::getter::get_frontend;

#[get("/")]
pub async fn get_home() -> Result<impl Responder, Error> {
    let new_content = get_frontend("home")?;
    Ok(HttpResponse::Ok().body(new_content))
}
