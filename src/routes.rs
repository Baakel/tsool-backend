use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Utc;
use surrealdb::{opt::PatchOp, Datetime};
use tracing::info;

use crate::models::{
    routes::{AddTodoRequest, CompleteTodoReq},
    AppState, SurrealTodo, Todo, TsoolError,
};

pub async fn session(State(state): State<Arc<AppState>>) -> Result<Json<String>, TsoolError> {
    let res: Option<String> = state.db.query("RETURN <String>$session").await?.take(0)?;

    Ok(Json(res.unwrap_or("No sesssion data found".into())))
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<AddTodoRequest>,
) -> Result<Json<String>, TsoolError> {
    // TODO: add a way to check if the todo alerady exists? Not sure we want to prevent duplicates
    // just yet
    info!(?todo, "we got some info");
    let res: Option<SurrealTodo> = state.db.create("todos").content(Todo::from(todo)).await?;
    info!(?res, "we got an answer");
    Ok(Json("ve von zulu!".to_string()))
}

pub async fn get_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SurrealTodo>>, TsoolError> {
    let todos: Vec<SurrealTodo> = state.db.select("todos").await?;
    Ok(Json(todos))
}

pub async fn delete_todo(State(state): State<Arc<AppState>>) -> Result<Json<String>, TsoolError> {
    Ok(Json("gtfo".to_string()))
}

pub async fn complete_todo(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<CompleteTodoReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = state
        .db
        .update(("todos", &todo.id))
        .patch(PatchOp::replace("/completed", Datetime::from(Utc::now())))
        .await?;
    Ok(Json(res))
}
