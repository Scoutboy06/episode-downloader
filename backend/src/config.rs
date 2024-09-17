use std::env;

use once_cell::sync::Lazy;

pub struct Config {
    pub frontend_path: String,
    pub gogo_gogoanime_cookie: String,
    pub gogo_auth_cookie: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv().ok();

    Config {
        frontend_path: env::var("FRONTEND_PATH"),
        gogo_gogoanime_cookie: env::var("GOGOANIME_COOKIE"),
        gogo_auth_cookie: env::var("GOGO_AUTH_COOKIE"),
    }
});
