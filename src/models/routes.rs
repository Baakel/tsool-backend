use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{Priority, Todo};

#[derive(Debug, Deserialize)]
pub struct AddTodoRequest {
    pub value: String,
    pub completed: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Priority,
}

impl From<AddTodoRequest> for Todo {
    fn from(value: AddTodoRequest) -> Self {
        Self {
            value: value.value,
            completed: value.completed.map(|v| v.into()),
            deadline: value.deadline.map(|v| v.into()),
            priority: value.priority,
            created: Utc::now().into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TodoReq {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct DeadlineReq {
    pub id: String,
    pub deadline: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PriorityReq {
    pub id: String,
    #[serde(default)]
    pub priority: Option<Priority>,
}
