use tsool_backend::{db::initialize_db, server::start_server};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt().pretty().init();
    let db = initialize_db().await.unwrap();
    start_server(db).await.unwrap();
}
