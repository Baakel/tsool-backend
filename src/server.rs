use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use surrealdb::{engine::any::Any, Surreal};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    models::{AppState, TsoolError},
    routes::{create_todo, session},
};

pub async fn start_server(db: Surreal<Any>) -> Result<(), TsoolError> {
    let listener = TcpListener::bind("0.0.0.0:9090").await.unwrap();
    let app_state = Arc::new(AppState { db });
    let router = Router::new()
        .route("/session", get(session))
        .route("/add_todo", post(create_todo))
        .with_state(app_state);
    info!("starting server on port 9090");
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
