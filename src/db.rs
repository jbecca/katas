use std::error::Error;

use sqlx::{Row, SqlitePool};

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
    info!("template table creation result: {:?}", template_table_result);

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

pub async fn list_n_katas(conn: &SqlitePool, number: &u32) -> Result<(), Box<dyn Error>> {
    let results =
        sqlx::query(r#"SELECT * from katas INNER JOIN attempts on katas.id = attempts.id LIMIT ?1;"#)
            .bind(number.to_string())
            .fetch_all(conn)
            .await?;

    println!(
        "{:>5} {:>24} {:>24} {:>15}",
        "entry", "name", "time", "difficulty"
    );
    for (idx, row) in results.iter().enumerate() {
        println!(
            "{:>5} {:>24} {:>24} {:>15}",
            idx,
            row.get::<String, &str>("name"),
            row.get::<String, &str>("time"),
            row.get::<String, &str>("difficulty")
        );
    }

    Ok(())
}
