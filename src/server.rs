use std::sync::Arc;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use surrealdb::{engine::any::Any, Surreal};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    models::{AppState, TsoolError},
    routes::{
        add_deadline, change_prio, complete_todo, create_todo, delete_todo, get_todos, session,
    },
};

pub async fn start_server(db: Surreal<Any>) -> Result<(), TsoolError> {
    let listener = TcpListener::bind("0.0.0.0:9090").await.unwrap();
    let app_state = Arc::new(AppState { db });
    let router = Router::new()
        .route("/session", get(session))
        .route("/add_todo", post(create_todo))
        .route("/get_todos", get(get_todos))
        .route("/complete", patch(complete_todo))
        .route("/todos", delete(delete_todo))
        .route("/todos/deadline", patch(add_deadline))
        .route("/todos/priority", patch(change_prio))
        .with_state(app_state);
    info!("starting server on port 9090");
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
