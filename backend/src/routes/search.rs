use actix_web::{get, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[get("/search")]
pub async fn search_gogo(info: web::Query<SearchParams>) -> HttpResponse {
    todo!()
}
