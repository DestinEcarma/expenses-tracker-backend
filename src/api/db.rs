pub mod defs;

use surrealdb::opt::auth::Root;

use self::defs::DB;

pub async fn get(secrets: shuttle_runtime::SecretStore) -> DB {
    let db = surrealdb::engine::any::connect(
        secrets.get("SURREAL_URL").expect("SURREAL_URL must be set"),
    )
    .await
    .expect("Could not connect to SurrealDB");

    let username = secrets
        .get("SURREAL_USER")
        .expect("DB_USERNAME must be set");
    let password = secrets
        .get("SURREAL_PASS")
        .expect("DB_PASSWORD must be set");

    db.signin(Root {
        username: username.as_str(),
        password: password.as_str(),
    })
    .await
    .expect("Could not sign into SurrealDB");

    let db_ns = secrets
        .get("SURREAL_NAMESPACE")
        .unwrap_or("dev".to_string());
    let db_db = secrets.get("SURREAL_DATABASE").unwrap_or("dev".to_string());

    db.use_ns(db_ns)
        .use_db(db_db)
        .await
        .expect("Could not use dev namespace and database");

    println!("Connected to SurrealDB");

    db
}
