use actix_web::{delete, web, Error, HttpResponse, Responder};

#[delete("/api/{db_title}/{table_title}")]
pub async fn delete_table(path: web::Path<(String, String)>) -> Result<impl Responder, Error> {
    let (db_title, table_title) = path.into_inner();
    Ok(HttpResponse::Ok().body(db_title + &table_title))
}
