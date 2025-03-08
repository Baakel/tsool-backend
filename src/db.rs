pub mod goals;
pub mod todos;

use surrealdb::{
    engine::any::{self, Any},
    opt::auth::Root,
    Surreal,
};
use tracing::info;

use crate::{config::CONFIG, models::TsoolError};

pub async fn initialize_db() -> Result<Surreal<Any>, TsoolError> {
    info!(url = &CONFIG.db_url, "Initializing DB connection");

    let db = any::connect(&CONFIG.db_url).await?;
    db.signin(Root {
        username: &CONFIG.user,
        password: &CONFIG.pass,
    })
    .await?;
    db.use_ns(&CONFIG.namespace)
        .use_db(&CONFIG.database)
        .await?;
    // DB.connect::<Wss>(&CONFIG.db_url).await?;
    // DB.use_ns("tsool").use_db("tasks").await?;
    // DB.use_ns("tsool").use_db("tasks").await?;
    //
    // DB.query(
    db.query(
        "
        DEFINE TABLE IF NOT EXISTS todos SCHEMAFULL
            PERMISSIONS FOR
                CREATE WHERE $auth,
                FOR UPDATE, SELECT, DELETE WHERE created_by = $auth;
        DEFINE FIELD IF NOT EXISTS value ON TABLE todos TYPE string;
        DEFINE FIELD IF NOT EXISTS completed ON TABLE todos TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS deadline ON TABLE todos TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS created ON TABLE todos TYPE datetime;
        DEFINE FIELD IF NOT EXISTS priority ON TABLE todos TYPE option<string>;
        DEFINE FIELD IF NOT EXISTS created_by ON TABLE todos VALUE $auth READONLY;

        DEFINE TABLE IF NOT EXISTS goals SCHEMAFULL
            PERMISSIONS FOR
                CREATE WHERE $auth,
                FOR UPDATE, SELECT, DELETE WHERE created_by = $auth;
        DEFINE FIELD IF NOT EXISTS value ON TABLE goals TYPE string;
        DEFINE FIELD IF NOT EXISTS completed ON TABLE goals TYPE option<datetime>;
        DEFINE FIELD IF NOT EXISTS created ON TABLE goals TYPE datetime;
        DEFINE FIELD IF NOT EXISTS created_by ON TABLE goals VALUE $auth READONLY;

        DEFINE TABLE IF NOT EXISTS users SCHEMAFULL;
        DEFINE FIELD IF NOT EXISTS name ON TABLE users TYPE string;
        DEFINE FIELD IF NOT EXISTS email ON TABLE users TYPE string ASSERT string::is::email($value);
        DEFINE FIELD IF NOT EXISTS password ON TABLE users TYPE string;
        
        DEFINE INDEX email ON users FIELDS email UNIQUE;

        DEFINE ACCESS IF NOT EXISTS users ON DATABASE TYPE RECORD
            SIGNIN (
                SELECT * FROM users WHERE email = $email AND crypto::argon2::compare(password, $password)
            )
            SIGNUP (
                CREATE users CONTENT {
                    name: $name,
                    email: $email,
                    password: crypto::argon2::generate($password)
                }
            );
        ",
    )
    .await?;
    Ok(db)
}
