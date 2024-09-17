use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use backend::config::CONFIG;
use routes::get_folders;
use serde::{Deserialize, Serialize};
use std::{env, ffi::OsString};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api").service(get_folders))
            .service(fs::Files::new("/series", &CONFIG.frontend_path).index_file("series.html"))
            .service(fs::Files::new("/", &CONFIG.frontend_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
