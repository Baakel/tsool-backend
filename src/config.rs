use std::sync::LazyLock;

pub struct Config {
    pub db_url: String,
    pub user: String,
    pub pass: String,
    pub api_key: String,
}

impl Config {
    pub fn init() -> Self {
        Config {
            db_url: std::env::var("DB_URL").unwrap(),
            user: std::env::var("DB_USER").unwrap(),
            pass: std::env::var("DB_PASS").unwrap(),
            api_key: std::env::var("API_KEY").unwrap(),
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::init);
