use actix_files as fs;
use actix_web::{web, App, HttpServer};
use backend::{
    config::CONFIG,
    routes::{get_folders, search_gogo},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .service(web::scope("/api").service(get_folders).service(search_gogo))
            .service(fs::Files::new("/series", &CONFIG.frontend_path).index_file("series.html"))
            .service(fs::Files::new("/", &CONFIG.frontend_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
