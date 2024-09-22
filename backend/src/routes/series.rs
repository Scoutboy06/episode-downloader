use actix_web::{get, web, HttpResponse};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};

use crate::scraping::{fetch_gogo, CollectTrim, SelectOne};

#[derive(Deserialize)]
struct GetSeriesDetailsQuery {
    id: String,
}

#[derive(Serialize)]
struct ResponseData<'a> {
    title: &'a str,
    poster_url: &'a str,
    series_type: &'a str,
    description: &'a str,
    genres: Vec<String>,
    released: &'a str,
    status: &'a str,
    other_name: String,
    episodes: Vec<EpisodeRange>,
}

#[derive(Serialize)]
struct EpisodeRange {
    start: i32,
    end: i32,
}

#[get("/series")]
pub async fn get_series_details(info: web::Query<GetSeriesDetailsQuery>) -> HttpResponse {
    let url = format!("https://anitaku.pe/category/{}", info.id);
    let response = fetch_gogo(url).await;

    let body = match response {
        Ok(body) => body.text().await.unwrap(),
        Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
    };

    let document = Html::parse_document(&body);

    let container = match document.select_one("#wrapper_bg > section > section.content_left > div.main_body > div:nth-child(2) > div.anime_info_body_bg") {
        Some(el) => el,
        None => {
            return HttpResponse::InternalServerError()
                .body("Scraped website returned invalid response")
        }
    };

    let title = container.select_one("h1").unwrap().text().collect_trimmed();
    let poster_url = container.select_one("img").unwrap().attr("src").unwrap();
    let series_type = container
        .select_one("p:nth-child(4) > a")
        .unwrap()
        .text()
        .collect_trimmed();
    let description = container
        .select_one(".description")
        .unwrap()
        .text()
        .collect_trimmed();
    let genres = container
        .select(&Selector::parse("p:nth-child(7) > a").unwrap())
        .map(|el| el.text().collect_trimmed())
        .collect::<Vec<_>>();
    let released = container
        .select_one("p:nth-child(8)")
        .unwrap()
        .text()
        .nth(1)
        .unwrap();
    let status = container
        .select_one("p:nth-child(9) > a")
        .unwrap()
        .text()
        .collect_trimmed();
    let other_name = container
        .select_one(&"p:nth-child(10) > a")
        .unwrap()
        .text()
        .collect_trimmed();
    let episodes = document
        .select(&Selector::parse("#episode_page > li > a").unwrap())
        .map(|el| EpisodeRange {
            start: el.attr("ep_start").unwrap().parse().unwrap(),
            end: el.attr("ep_end").unwrap().parse().unwrap(),
        })
        .collect::<Vec<_>>();

    HttpResponse::Ok().json(ResponseData {
        title: &title,
        poster_url,
        series_type: &series_type,
        description: &description,
        genres,
        released: &released,
        status: &status,
        other_name,
        episodes,
    })
}
