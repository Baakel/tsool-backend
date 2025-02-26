use surrealdb::{
    engine::{
        any::{self, Any},
        remote::ws::{Client, Wss},
    },
    opt::auth::Root,
    Surreal,
};
use tracing::info;

use crate::{
    config::CONFIG,
    models::{Goal, Todo, TsoolError, DB},
};

pub async fn upsert_todo(todo: &Todo) -> Result<(), TsoolError> {
    todo!()
}

pub async fn get_todos() -> Result<Vec<Todo>, TsoolError> {
    todo!()
}

pub async fn upsert_goal(goal: &Goal) -> Result<(), TsoolError> {
    todo!()
}

pub async fn get_todays_goal() -> Result<Goal, TsoolError> {
    todo!()
}

pub async fn initialize_db() -> Result<Surreal<Any>, TsoolError> {
    info!(url = &CONFIG.db_url, "Initializing DB connection");

    let db = any::connect(&CONFIG.db_url).await?;
    db.signin(Root {
        username: &CONFIG.user,
        password: &CONFIG.pass,
    })
    .await?;
    db.use_ns("tsool").use_db("tasks").await?;
    // DB.connect::<Wss>(&CONFIG.db_url).await?;
    // DB.use_ns("tsool").use_db("tasks").await?;
    // DB.use_ns("tsool").use_db("tasks").await?;
    //
    // DB.query(
    db.query(
        "
        DEFINE TABLE IF NOT EXISTS todos SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS value ON TABLE todos TYPE string;
        DEFINE FIELD IF NOT EXISTS completed ON TABLE todos TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS deadline ON TABLE todos TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS created ON TABLE todos TYPE datetime;
        DEFINE FIELD IF NOT EXISTS priority ON TABLE todos TYPE option<string>;

        DEFINE TABLE IF NOT EXISTS goals SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS value ON TABLE goals TYPE string;
        DEFINE FIELD IF NOT EXISTS completed ON TABLE goals TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS created ON TABLE goals TYPE datetime;
        ",
    )
    .await?;
    Ok(db)
}
