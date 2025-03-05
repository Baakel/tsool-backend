use std::{path::Path, sync::LazyLock};

use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Config {
    pub db_url: String,
    pub user: String,
    pub pass: String,
    pub api_key: String,
    pub token_secret: Vec<u8>,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey,
}

impl Config {
    pub fn init() -> Self {
        let encoding_key_path = format!(
            "{}/ec_tsool.pem",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        let decoding_key_path = format!(
            "{}/ec_tsool_pub.pem",
            std::env::var("CARGO_MANIFEST_DIR").unwrap()
        );
        let key_bytes = std::fs::read(encoding_key_path).unwrap();
        let decoding_bytes = std::fs::read(decoding_key_path).unwrap();
        let encoding_key = EncodingKey::from_ec_pem(&key_bytes).unwrap();
        let decoding_key = DecodingKey::from_ec_pem(&decoding_bytes).unwrap();
        Config {
            db_url: std::env::var("DB_URL").unwrap(),
            user: std::env::var("DB_USER").unwrap(),
            pass: std::env::var("DB_PASS").unwrap(),
            api_key: std::env::var("API_KEY").unwrap(),
            token_secret: vec![],
            encoding_key,
            decoding_key,
        }
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::init);
