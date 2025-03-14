use chrono::Utc;
use surrealdb::{engine::any::Any, opt::PatchOp, Datetime, Surreal};
use tracing::debug;

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

pub async fn complete_goal(
    db: &Surreal<Any>,
    goal_id: &str,
) -> Result<Option<SurrealGoal>, TsoolError> {
    let res: Option<SurrealGoal> = db
        .update(("goals", goal_id))
        .patch(PatchOp::replace("/completed", Datetime::from(Utc::now())))
        .await?;
    Ok(res)
}

pub async fn get_todays_goal(db: &Surreal<Any>) -> Result<Option<SurrealGoal>, TsoolError> {
    let mut res = db
        .query("SELECT * FROM goals WHERE created > time::group($date, \"day\")")
        .bind(("date", Datetime::from(Utc::now())))
        .await?;
    debug!(?res, "res from db");
    let goal: Option<SurrealGoal> = res.take(0)?;
    Ok(goal)
}
