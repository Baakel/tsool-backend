use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use chrono::Utc;
use surrealdb::{opt::PatchOp, Datetime};
use tracing::info;

use crate::models::{
    routes::{AddTodoRequest, DeadlineReq, PriorityReq, TodoReq},
    AppState, SurrealTodo, Todo, TsoolError,
};

pub async fn session(State(state): State<Arc<AppState>>) -> Result<Json<String>, TsoolError> {
    let res: Option<String> = state.db.query("RETURN <String>$session").await?.take(0)?;
    Ok(Json(res.unwrap_or("No sesssion data found".into())))
}

pub async fn create_todo(
    State(state): State<Arc<AppState>>,
    Json(todo): Json<AddTodoRequest>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    // TODO: add a way to check if the todo alerady exists? Not sure we want to prevent duplicates
    // just yet
    let res: Option<SurrealTodo> = state.db.create("todos").content(Todo::from(todo)).await?;
    Ok(Json(res))
}

pub async fn get_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SurrealTodo>>, TsoolError> {
    let todos: Vec<SurrealTodo> = state.db.select("todos").await?;
    Ok(Json(todos))
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<TodoReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = state.db.delete(("todos", &todo.id)).await?;
    Ok(Json(res))
}

pub async fn complete_todo(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<TodoReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = state
        .db
        .update(("todos", &todo.id))
        .patch(PatchOp::replace("/completed", Datetime::from(Utc::now())))
        .await?;
    Ok(Json(res))
}

pub async fn add_deadline(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<DeadlineReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = state
        .db
        .update(("todos", &todo.id))
        .patch(PatchOp::replace(
            "/deadline",
            todo.deadline.map(Datetime::from),
        ))
        .await?;
    Ok(Json(res))
}

pub async fn change_prio(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<PriorityReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    info!(?todo, "we got a todo");
    let res = state
        .db
        .update(("todos", &todo.id))
        .patch(PatchOp::replace("/priority", todo.priority))
        .await?;
    Ok(Json(res))
}
