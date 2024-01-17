use std::error::Error;

use sqlx::{Row, SqlitePool};

pub async fn setup_tables(pool: &SqlitePool) -> Result<(), Box<dyn Error>> {
    let mut conn = pool.acquire().await?;
    let name_table_result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS 
        katas (id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(140) NOT NULL);",
    )
    .execute(&mut *conn)
    .await
    .unwrap();
    println!("Result: {:?}", name_table_result);

    let status_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        status (id INTEGER, status BOOL,
        FOREIGN KEY(id) REFERENCES katas(id))"#
    )
    .execute(pool)
    .await
    .unwrap();

    println!("Result: {:?}", status_table_result);
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

pub async fn list_n_katas(conn: &SqlitePool, number: &u32) {
    let results = sqlx::query(r#"SELECT * from katas LIMIT ?1;"#)
        .bind(number.to_string())
        .fetch_all(conn)
        .await
        .unwrap();

    for (idx, row) in results.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
}
