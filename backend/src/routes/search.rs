use actix_web::{get, web, HttpResponse};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::scraping::fetch_gogo;

#[derive(Deserialize)]
struct SearchParams {
    q: String,
}

#[derive(Deserialize)]
struct GogoSearchResponse {
    content: String,
}

#[derive(Serialize)]
struct SearchResponseItem {
    id: String,
    name: String,
    img: String,
}

#[get("/search")]
pub async fn search_gogo(info: web::Query<SearchParams>) -> HttpResponse {
    let url = format!(
        "https://ajax.gogocdn.net/site/loadAjaxSearch?keyword={}",
        info.q
    );
    let response = fetch_gogo(url).await;

    let json = match response {
        Ok(res) => res.json::<GogoSearchResponse>().await,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let body = match json {
        Ok(json) => json.content,
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let document = Html::parse_fragment(&body);

    let mut response_items: Vec<SearchResponseItem> = Vec::new();

    let items_selector = Selector::parse(".list_search_ajax").unwrap();
    let items = document.select(&items_selector);

    for item in items {
        let a = item.child_elements().next().unwrap();
        let id = a.attr("href").unwrap()["category/".len()..].to_owned();

        let inner_div = a.child_elements().next().unwrap();
        let img = extract_url(inner_div.attr("style").unwrap())
            .unwrap()
            .to_owned();

        response_items.push(SearchResponseItem {
            id,
            name: a.text().collect(),
            img,
        })
    }

    HttpResponse::Ok().body(serde_json::to_string(&response_items).unwrap())
}

fn extract_url(input: &str) -> Option<&str> {
    let start = input.find('"')? + 1;
    let end = input[start..].find('"')? + start;
    Some(&input[start..end])
}
