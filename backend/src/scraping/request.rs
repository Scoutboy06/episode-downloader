use crate::config::CONFIG;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client, Error, Response,
};

pub async fn fetch_gogo(url: String) -> Result<Response, Error> {
    let client = Client::new();
    client.get(url).headers(construct_headers()).send().await
}

fn construct_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        HeaderName::from_static("accept-language"),
        HeaderValue::from_static("en-US,en;q=0.9,sv;q=0.8"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua"),
        HeaderValue::from_static(
            r#"Chromium";v="128", "Not;A=Brand";v="24", "Microsoft Edge";v="128"#,
        ),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-mobile"),
        HeaderValue::from_static("?0"),
    );
    headers.insert(
        HeaderName::from_static("sec-ch-ua-platform"),
        HeaderValue::from_static(r#""Windows""#),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-dest"),
        HeaderValue::from_static("document"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-mode"),
        HeaderValue::from_static("navigate"),
    );
    headers.insert(
        HeaderName::from_static("sec-fetch-site"),
        HeaderValue::from_static("cross-site"),
    );
    headers.insert(
        HeaderName::from_static("cookie"),
        HeaderValue::from_str(&auth_cookies()).unwrap(),
    );

    headers
}

fn auth_cookies() -> String {
    format!(
        "gogoanime={}; auth={}",
        CONFIG.gogo_gogoanime_cookie, CONFIG.gogo_auth_cookie
    )
}
