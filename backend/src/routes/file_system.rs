use actix_web::{get, web, HttpResponse};
use serde::{Deserialize, Serialize};

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
pub async fn get_folders(info: web::Query<PathParams>) -> HttpResponse {
    let mut directories: Vec<DirEntry> = Vec::new();

    let entries = {
        let result = std::fs::read_dir(&info.path);
        if result.is_err() {
            return HttpResponse::InternalServerError().body("Path not found");
        }
        result.unwrap()
    };

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
