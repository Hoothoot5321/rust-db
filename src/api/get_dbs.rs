use std::fs;

use actix_web::{get, Error, HttpResponse, Responder};

use crate::classes::{err::create_err, test::Test};

#[get("/api")]
async fn get_dbs() -> Result<impl Responder, Error> {
    let temp_paths = fs::read_dir("./storage/".to_owned());

    let paths = match temp_paths {
        Ok(dir) => dir,
        Err(_) => return Err(create_err("WTF".to_owned(), 400).into()),
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
