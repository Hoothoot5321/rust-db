use std::fs;

use actix_web::{post, web, Error, HttpResponse, Responder};
use serde::Deserialize;

use crate::{classes::err::create_err, helper::convo_json, misc::base_path};

#[derive(Debug, Deserialize)]
pub struct TableHeaders {
    name: String,
    headers: Vec<String>,
}
#[post("/api/{db_title}")]
pub async fn post_table(
    path: web::Path<String>,
    payload: web::Payload,
) -> Result<impl Responder, Error> {
    let db_title = path.into_inner();
    let bytes = payload.to_bytes().await?;

    let body = String::from_utf8(bytes.to_vec()).unwrap();

    let obj = match convo_json::<TableHeaders>(&body) {
        Ok(content) => content,
        Err(err) => return Err(err.into()),
    };
    let file_path = base_path() + &db_title + "/" + &obj.name + ".csv";

    match fs::metadata(&file_path) {
        Err(_) => {}
        Ok(_) => {
            return Err(create_err(
                "Table [".to_owned()
                    + &obj.name
                    + "] in database ["
                    + &db_title
                    + "] aldready exists",
                400,
            )
            .into());
        }
    }

    let temp_wtr = csv::Writer::from_path(file_path);

    let mut wtr = match temp_wtr {
        Ok(writer) => writer,
        Err(_) => {
            return Err(create_err(
                "Erorr opening table [".to_owned()
                    + &obj.name
                    + "] in database ["
                    + &db_title
                    + "]",
                400,
            )
            .into())
        }
    };
    wtr.write_record(obj.headers).unwrap();
    wtr.flush()?;

    Ok(HttpResponse::Ok().body(db_title))
}
