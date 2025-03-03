use chrono::{DateTime, Utc};
use surrealdb::{engine::any::Any, opt::PatchOp, Datetime, Surreal};

use crate::models::{Priority, SurrealTodo, Todo, TsoolError};

pub async fn get_db_todos(db: &Surreal<Any>) -> Result<Vec<SurrealTodo>, TsoolError> {
    let todos: Vec<SurrealTodo> = db.select("todos").await?;
    Ok(todos)
}

pub async fn create_db_todo(
    db: &Surreal<Any>,
    todo: Todo,
) -> Result<Option<SurrealTodo>, TsoolError> {
    let res: Option<SurrealTodo> = db.create("todos").content(todo).await?;
    Ok(res)
}

pub async fn delete_db_todo(
    db: &Surreal<Any>,
    todo_id: &str,
) -> Result<Option<SurrealTodo>, TsoolError> {
    let res: Option<SurrealTodo> = db.delete(("todos", todo_id)).await?;
    Ok(res)
}

pub async fn complete_todo(
    db: &Surreal<Any>,
    todo_id: &str,
) -> Result<Option<SurrealTodo>, TsoolError> {
    let res: Option<SurrealTodo> = db
        .update(("todos", todo_id))
        .patch(PatchOp::replace("/completed", Datetime::from(Utc::now())))
        .await?;
    Ok(res)
}

pub async fn update_todo_deadline(
    db: &Surreal<Any>,
    todo_id: &str,
    deadline: Option<DateTime<Utc>>,
) -> Result<Option<SurrealTodo>, TsoolError> {
    let res: Option<SurrealTodo> = db
        .update(("todos", todo_id))
        .patch(PatchOp::replace("/deadline", deadline.map(Datetime::from)))
        .await?;
    Ok(res)
}

pub async fn update_todo_priority(
    db: &Surreal<Any>,
    todo_id: &str,
    priority: Option<Priority>,
) -> Result<Option<SurrealTodo>, TsoolError> {
    let res: Option<SurrealTodo> = db
        .update(("todos", todo_id))
        .patch(PatchOp::replace("/priority", priority))
        .await?;
    Ok(res)
}
