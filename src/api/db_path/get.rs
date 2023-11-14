use std::fs;

use actix_web::{get, web, Error, HttpResponse, Responder};
use serde_json::{Map, Number, Value};

use crate::{classes::err::create_err, misc::base_path};

#[get("/api/{db_title}/{table_title}")]
pub async fn get_table(path: web::Path<(String, String)>) -> Result<impl Responder, Error> {
    let (db_title, table_title) = path.into_inner();

    let file_path = base_path() + &db_title + "/" + &table_title + ".csv";

    let content = fs::read_to_string(file_path)?;

    let mut reader = csv::Reader::from_reader(content.as_bytes());

    let temp_headers = reader.headers().cloned();

    let headers = match temp_headers {
        Ok(headers) => headers,
        Err(_) => {
            return Err(create_err("Damn".to_owned(), 400).into());
        }
    };

    let records: Vec<_> = reader.records().collect();

    let mut temp_resp_body = Map::new();

    for (i, header) in (&headers).iter().enumerate() {
        let mut temp_arr = vec![];

        for record in &records {
            let record = record.as_ref().unwrap();
            let fisk = record[i].to_owned();
            match fisk.parse::<u16>() {
                Ok(num) => temp_arr.push(Value::Number(Number::from(num))),
                Err(_) => temp_arr.push(Value::String(fisk)),
            }
        }

        temp_resp_body.insert(header.to_owned(), Value::Array(temp_arr));
    }

    let bang = serde_json::to_string(&temp_resp_body).unwrap();

    Ok(HttpResponse::Ok().body(bang))
}
