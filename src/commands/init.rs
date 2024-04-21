use std::error::Error;
use lib_katas::util;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{Sqlite, SqlitePool},
};


pub(crate) async fn run() -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::init::run");
    let user_cfg = util::parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        if !Sqlite::database_exists(&format!("sqlite://{loc}"))
            .await
            .unwrap_or(false)
        {
            match Sqlite::create_database(&format!("sqlite://{loc}")).await {
                Ok(_) => {
                    println!("DB created");
                    let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
                    setup_tables(&pool).await?;
                    pool.close().await;
                }
                Err(err) => panic!("error: {}", err),
            }
        } else {
            let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
            setup_tables(&pool).await?;
            pool.close().await;
        }
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}

pub async fn setup_tables(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.acquire().await?;
    let name_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        katas (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        name TEXT NOT NULL UNIQUE);"#,
    )
    .execute(&mut *conn)
    .await?;
    info!("kata name table creation result: {:?}", name_table_result);

    let status_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        status (
        id INTEGER NOT NULL, 
        due TEXT NOT NULL,
        n_success INTEGER NOT NULL,
        last_interval TEXT NOT NULL,
        FOREIGN KEY (id) REFERENCES katas(id)
        ON DELETE CASCADE ON UPDATE CASCADE);"#,
    )
    .execute(pool)
    .await?;
    info!("status table creation result: {:?}", status_table_result);

    let template_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        rust (
        id INTEGER NOT NULL,
        main TEXT NOT NULL,
        cargo TEXT NOT NULL,
        FOREIGN KEY (id) REFERENCES katas(id)
        ON DELETE CASCADE ON UPDATE CASCADE);"#,
    )
    .execute(pool)
    .await?;
    info!(
        "template table creation result: {:?}",
        template_table_result
    );

    let attempts_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        attempts (
        id INTEGER NOT NULL,
        difficulty TEXT NOT NULL,
        time TEXT NOT NULL,
        FOREIGN KEY (id) REFERENCES katas(id)
        ON DELETE CASCADE ON UPDATE CASCADE);"#,
    )
    .execute(pool)
    .await?;

    info!("Attempt table creation result: {:?}", attempts_table_result);

    Ok(())
}

pub async fn insert_kata_name(conn: &SqlitePool, kata_name: String) {
    let result = sqlx::query(r#"INSERT into katas (name) VALUES ( ?1 );"#)
        .bind(kata_name.as_str())
        .execute(conn)
        .await
        .unwrap();

    println!("Result: {:?}", result);
}

