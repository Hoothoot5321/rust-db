use std::fs;

use actix_web::{get, web, Error, HttpResponse, Responder};

use crate::classes::{err::create_err, test::Test};

#[get("/api/{db_title}")]
async fn get_db(path: web::Path<String>) -> Result<impl Responder, Error> {
    let db_title = path;

    let temp_paths = fs::read_dir("./storage/".to_owned() + &db_title);

    let paths = match temp_paths {
        Ok(dir) => dir,
        Err(_) => {
            return Err(create_err(
                "Database: [".to_owned() + &db_title + "] does not exist",
                400,
            )
            .into())
        }
    };

    let quick: Vec<String> = paths
        .into_iter()
        .map(|x| match x.unwrap().file_name().to_str() {
            Some(nice) => nice.to_owned(),
            None => "".to_owned(),
        })
        .collect();

    let test = serde_json::to_string(&quick)?;

    let damn = serde_json::to_string(&Test { msg: test })?;

    Ok(HttpResponse::Ok().body(damn))
}
