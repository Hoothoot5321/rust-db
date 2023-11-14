use std::fs;

use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::Deserialize;

use crate::classes::err::create_err;
use crate::classes::test::Test;
use crate::helper::convo_json;

use crate::misc::base_path;

#[derive(Debug, Deserialize)]
pub struct NewMsg {
    title: String,
}

#[post("/api/")]
async fn post_db(payload: web::Payload) -> Result<impl Responder, Error> {
    let bytes = payload.to_bytes().await?;

    let body = String::from_utf8(bytes.to_vec()).unwrap();

    let obj = match convo_json::<NewMsg>(&body) {
        Ok(content) => content,
        Err(err) => return Err(err.into()),
    };

    match fs::create_dir(base_path() + &obj.title) {
        Ok(_) => {}
        Err(_) => {
            return Err(create_err(
                "Database: [".to_owned() + &obj.title + "] already exists",
                400,
            )
            .into())
        }
    }

    let resp_body = serde_json::to_string(&Test {
        msg: "Succes".to_owned(),
    })?;

    Ok(HttpResponse::Ok().body(resp_body))
}
