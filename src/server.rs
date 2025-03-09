use std::sync::Arc;

use axum::{
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Router,
};
use surrealdb::{engine::any::Any, Surreal};
use tokio::net::TcpListener;
use tracing::info;

use crate::{
    middleware::auth_middleware,
    models::{AppState, TsoolError},
    routes::{
        authorize, authorize_app,
        goals::create_goal,
        signup,
        todos::{
            add_deadline, change_prio, complete_todo_handler, create_todo, delete_todo, get_todos,
            session,
        },
    },
};

pub async fn start_server(db: Surreal<Any>) -> Result<(), TsoolError> {
    let listener = TcpListener::bind("0.0.0.0:9090").await.unwrap();
    let app_state = Arc::new(AppState { db });
    let router = Router::new()
        .route("/session", get(session))
        .route("/add_todo", post(create_todo))
        .route("/get_todos", get(get_todos))
        .route("/complete", patch(complete_todo_handler))
        .route("/todos", delete(delete_todo))
        .route("/todos/deadline", patch(add_deadline))
        .route("/todos/priority", patch(change_prio))
        .route("/authorize", post(authorize))
        .route("/authorize_app", post(authorize_app))
        .route("/signup", post(signup))
        .route("/goal", post(create_goal))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ))
        .with_state(app_state);
    let router = router.fallback(handler_404);
    info!("starting server on port 9090");
    axum::serve(listener, router).await.unwrap();
    Ok(())
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "¯\\_( ͡° ͜ʖ ͡°)_/¯")
}
