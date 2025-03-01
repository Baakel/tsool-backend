use std::{fmt::Display, sync::LazyLock};

use axum::{http::StatusCode, response::IntoResponse, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::{
    engine::{any::Any, remote::ws::Client},
    Datetime, RecordId, Surreal,
};
use thiserror::Error;

pub mod routes;

pub struct AppState {
    pub db: Surreal<Any>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Todo {
    pub value: String,
    pub completed: Option<Datetime>,
    pub created: Datetime,
    pub deadline: Option<Datetime>,
    pub priority: Priority,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SurrealTodo {
    pub id: RecordId,
    pub value: String,
    pub completed: Option<Datetime>,
    pub created: Datetime,
    pub deadline: Option<Datetime>,
    pub priority: Option<Priority>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Goal {
    pub value: String,
    pub completed: Option<Datetime>,
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

pub static DB: LazyLock<Surreal<Client>> = LazyLock::new(Surreal::init);

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
            created: now.into(),
            deadline: deadline.map(|v| v.into()),
            priority: priority.unwrap_or(Priority::Low),
        }
    }
}

impl Task for Todo {
    fn complete(&mut self) {
        self.completed = Some(Utc::now().into());
    }

    fn uncomplete(&mut self) {
        self.completed = None;
    }
}

impl Task for Goal {
    fn complete(&mut self) {
        self.completed = Some(Utc::now().into());
    }

    fn uncomplete(&mut self) {
        self.completed = None;
    }
}

trait Task {
    fn complete(&mut self);
    fn uncomplete(&mut self);
}

impl Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_rep = match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
            Priority::Urgent => "urgent",
            Priority::Unknown => "unknown",
        };
        write!(f, "{str_rep}")
    }
}
