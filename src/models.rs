use std::sync::LazyLock;

use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{engine::any::Any, engine::remote::ws::Client, Surreal};
use thiserror::Error;

pub struct AppState {
    pub db: Surreal<Any>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub value: String,
    pub completed: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Priority,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Goal {
    pub value: String,
    pub completed: Option<DateTime<Utc>>,
    pub created: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
    Unknown,
}

#[derive(Debug, Error)]
pub enum TsoolError {
    #[error("Error while talking to DB: {0}")]
    DB(#[from] surrealdb::Error),
}

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(|| {
    let db = Surreal::init();
    db
});

impl IntoResponse for TsoolError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(self.to_string())).into_response()
    }
}

impl Todo {
    fn new(value: &str, end: Option<DateTime<Utc>>, priority: Option<Priority>) -> Self {
        let now = Utc::now();
        let mut deadline = end;
        if let Some(dl) = end {
            if dl > now {
                deadline = None;
            }
        }
        Self {
            value: value.to_string(),
            completed: None,
            created: now,
            deadline,
            priority: priority.unwrap_or(Priority::Low),
        }
    }
}

impl Task for Todo {
    fn complete(&mut self) {
        self.completed = Some(Utc::now());
    }

    fn uncomplete(&mut self) {
        self.completed = None;
    }
}

impl Task for Goal {
    fn complete(&mut self) {
        self.completed = Some(Utc::now());
    }

    fn uncomplete(&mut self) {
        self.completed = None;
    }
}

trait Task {
    fn complete(&mut self);
    fn uncomplete(&mut self);
}
