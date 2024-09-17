use std::env;

use actix_web::http::header::HeaderMap;

pub async fn fetch_gogo(path: String) -> Result<String, ()> {

    // reqwest::get(format!("https://ajax.gogocdn.net/site/loadAjaxSearch"))
    //     .await?
    //     .text()
    //     .await?
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("accept-language", "en-US,en;q=0.9,sv;q=0.8");
    headers.insert(
        "sec-ch-ua",
        r#"Chromium";v="128", "Not;A=Brand";v="24", "Microsoft Edge";v="128"#,
    );
    headers.insert("sec-ch-ua-mobile", "?0");
    headers.insert("sec-ch-ua-platform", r#""Windows""#);
    headers.insert("sec-fetch-dest", "document");
    headers.insert("sec-fetch-mode", "navigate");
    headers.insert("sec-fetch-site", "cross-site");

    headers.insert("cookie", auth_cookies());

    headers
}

fn auth_cookies() -> String {
    let gogo_token = env::var("gogo_cookie_token").ex;
    format!()
}
