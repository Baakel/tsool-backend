use std::sync::Arc;

use axum::{extract::State, Json};

use crate::{
    db::goals::create_db_goal,
    models::{AppState, Goal, TsoolError},
};

pub async fn create_goal(
    State(state): State<Arc<AppState>>,
    Json(goal): Json<Goal>,
) -> Result<(), TsoolError> {
    let res = create_db_goal(&state.db, goal).await?;
    Ok(())
}
