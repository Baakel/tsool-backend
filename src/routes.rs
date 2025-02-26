use std::sync::Arc;

use axum::{extract::State, Json};
use tracing::info;

use crate::models::{AppState, Todo, TsoolError};

pub async fn session(State(state): State<Arc<AppState>>) -> Result<Json<String>, TsoolError> {
    let res: Option<String> = state.db.query("RETURN <String>$session").await?.take(0)?;

    Ok(Json(res.unwrap_or("No sesssion data found".into())))
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<Todo>,
) -> Result<Json<String>, TsoolError> {
    // TODO: add a way to check if the todo alerady exists? Not sure we want to prevent duplicates
    // just yet
    info!(?todo, "we got some info");
    let res: Option<Todo> = state.db.create("todo").content(todo).await?;
    info!(?res, "we got an answer");
    Ok(Json("ve von zulu!".to_string()))
}
