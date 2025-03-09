use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use tracing::info;

use crate::{
    db::todos::{
        complete_todo, create_db_todo, delete_db_todo, get_db_todos, update_todo_deadline,
        update_todo_priority,
    },
    models::{
        routes::{AddTodoRequest, DeadlineReq, PriorityReq, TodoReq},
        AppState, SurrealTodo, Todo, TsoolError,
    },
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
    let todo = Todo::from(todo);
    let res = create_db_todo(&state.db, todo).await?;
    Ok(Json(res))
}

pub async fn get_todos(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<SurrealTodo>>, TsoolError> {
    let todos = get_db_todos(&state.db).await?;
    Ok(Json(todos))
}

pub async fn delete_todo(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<TodoReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = delete_db_todo(&state.db, &todo.id).await?;
    Ok(Json(res))
}

pub async fn complete_todo_handler(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<TodoReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = complete_todo(&state.db, &todo.id).await?;
    Ok(Json(res))
}

pub async fn add_deadline(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<DeadlineReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    let res: Option<SurrealTodo> = update_todo_deadline(&state.db, &todo.id, todo.deadline).await?;
    Ok(Json(res))
}

pub async fn change_prio(
    State(state): State<Arc<AppState>>,
    Query(todo): Query<PriorityReq>,
) -> Result<Json<Option<SurrealTodo>>, TsoolError> {
    info!(?todo, "we got a todo");
    let res = update_todo_priority(&state.db, &todo.id, todo.priority).await?;
    Ok(Json(res))
}
