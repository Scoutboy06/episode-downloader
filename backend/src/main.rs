use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpServer};
use backend::{
    config::CONFIG,
    routes::{get_folders, get_series_details, search_gogo},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:5173")
            .allowed_origin("http://localhost:5173")
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(
                web::scope("/api")
                    .service(get_folders)
                    .service(search_gogo)
                    .service(get_series_details),
            )
            .service(fs::Files::new("/", &CONFIG.frontend_path).index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
