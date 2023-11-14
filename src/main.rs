use actix_web::{App, HttpServer};

pub mod frontend;

mod classes;

mod api;

pub mod home;

pub mod helper;

use home::get_home;

use api::{
    db_path::{get::get_table, post::post_table, put::put_table},
    get::get_db,
    get_dbs::get_dbs,
    post::post_db,
};
use testo::testo;

mod misc;

mod testo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let test = true;

    if test {
        testo();
    }

    HttpServer::new(|| {
        App::new()
            .service(get_dbs)
            .service(get_home)
            .service(post_db)
            .service(get_db)
            .service(get_table)
            .service(post_table)
            .service(put_table)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
