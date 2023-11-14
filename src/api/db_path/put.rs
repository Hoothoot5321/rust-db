use actix_web::{put, web, Error, HttpResponse, Responder};
use serde_json::{Map, Value};

use crate::{classes::err::create_err, misc::base_path};

#[put("/api/{db_title}/{table_title}")]
async fn put_table(
    path: web::Path<(String, String)>,
    payload: web::Payload,
) -> Result<impl Responder, Error> {
    let (db_title, table_title) = path.into_inner();
    let bytes = payload.to_bytes().await?;

    let body = String::from_utf8(bytes.to_vec()).unwrap();

    let parsed = serde_json::from_str::<Map<String, Value>>(&body)?;

    let file_path = base_path() + &db_title + "/" + &table_title + ".csv";

    let temp_wtr = csv::Writer::from_path(file_path);

    let mut wtr = match temp_wtr {
        Ok(writer) => writer,
        Err(_) => {
            return Err(create_err(
                "Erorr opening table [".to_owned()
                    + &table_title
                    + "] in database ["
                    + &db_title
                    + "]",
                400,
            )
            .into())
        }
    };

    let mut keys = vec![];

    let mut vals = vec![];

    for (key, val) in parsed.iter() {
        keys.push(key);

        let arr = match val.as_array() {
            Some(arr) => arr,
            None => {
                return Err(create_err("Dunno".to_owned(), 400).into());
            }
        };

        let fish: Vec<&str> = arr
            .iter()
            .map(|val| match val.as_str() {
                Some(nice) => nice,
                None => "",
            })
            .collect();
        vals.push(fish);
    }

    wtr.write_record(&keys).unwrap();

    for i in 0..vals[0].len() {
        let mut row = vec![];
        for b in 0..vals.len() {
            row.push(vals[b][i]);
        }
        wtr.write_record(row).unwrap();
    }

    wtr.flush().unwrap();

    Ok(HttpResponse::Ok().body("Test"))
}
