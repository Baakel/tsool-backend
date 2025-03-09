use surrealdb::{engine::any::Any, Surreal};

use crate::models::{Goal, SurrealGoal, TsoolError};

pub async fn create_db_goal(
    db: &Surreal<Any>,
    goal: Goal,
) -> Result<Option<SurrealGoal>, TsoolError> {
    let res = db.create("goals").content(goal).await?;
    Ok(res)
}

pub async fn delete_db_goal(
    db: &Surreal<Any>,
    goal_id: &str,
) -> Result<Option<SurrealGoal>, TsoolError> {
    let res: Option<SurrealGoal> = db.delete(("goals", goal_id)).await?;
    Ok(res)
}
