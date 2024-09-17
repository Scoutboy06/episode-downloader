#![allow(unused)]

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::{env, ffi::OsString};

#[derive(Deserialize)]
struct PathParams {
    path: String,
}

#[derive(Serialize)]
struct DirEntry {
    name: String,
    path: String,
}

#[get("/dirs")]
async fn get_path_dirs(info: web::Query<PathParams>) -> impl Responder {
    let mut directories: Vec<DirEntry> = Vec::new();

    // if info.path.is_empty() {}

    let entries = {
        let result = std::fs::read_dir(&info.path);
        if result.is_err() {
            return HttpResponse::InternalServerError().body("Path not found");
        }
        result.unwrap()
    };

    dbg!(&entries);

    for entry in entries {
        match entry {
            Ok(entry) => match entry.file_type() {
                Ok(file_type) => {
                    if file_type.is_dir() {
                        directories.push(DirEntry {
                            name: entry.file_name().into_string().unwrap(),
                            path: entry.path().to_string_lossy().to_string(),
                        });
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    HttpResponse::Ok().json(directories)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let frontend_path = env::var("FRONTEND_PATH").expect("Env FRONTEND_PATH not defined");
    dbg!(&frontend_path);

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api").service(get_path_dirs))
            .service(fs::Files::new("/series", &frontend_path).index_file("series.html"))
            .service(fs::Files::new("/", &frontend_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
