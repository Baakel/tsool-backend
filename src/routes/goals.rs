use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    db::goals::{create_db_goal, get_todays_goal},
    models::{routes::GoalInput, AppState, Goal, SurrealGoal, TsoolError},
};

pub async fn create_goal(
    State(state): State<Arc<AppState>>,
    Json(goal): Json<GoalInput>,
) -> Result<Json<Option<SurrealGoal>>, TsoolError> {
    let res = create_db_goal(&state.db, Goal::new(goal.value, None)).await?;
    Ok(Json(res))
}

pub async fn get_todays_goal_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Option<SurrealGoal>>, TsoolError> {
    let res = get_todays_goal(&state.db).await?;
    Ok(Json(res))
}
