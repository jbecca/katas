use std::error::Error;

use crate::util::Language;
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
    println!("Result: {:?}", name_table_result);

    let status_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        status (
        id INTEGER NOT NULL, 
        time TEXT NOT NULL,
        language TEXT NOT NULL,
        FOREIGN KEY (id) REFERENCES katas(id)
        ON DELETE CASCADE ON UPDATE CASCADE);"#,
    )
    .execute(pool)
    .await?;
    println!("Result: {:?}", status_table_result);

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
    println!("Result: {:?}", template_table_result);

    let attempts_table_result = sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS
        attempts (
        id INTEGER NOT NULL,
        difficulty TEXT NOT NULL,
        time TEXT NOT NULL
        FOREIGN KEY (id) REFERENCES katas(id)
        ON DELETE CASCADE ON UPDATE CASCADE);"#
    )
    .execute(pool)
    .await?;

    println!("Result: {:?}", attempts_table_result);

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
        sqlx::query(r#"SELECT * from katas INNER JOIN status on katas.id = status.id LIMIT ?1;"#)
            .bind(number.to_string())
            .fetch_all(conn)
            .await?;

    println!(
        "{:>4} {:>24} {:>24} {:>10} ",
        "id", "name", "time", "language"
    );
    for (idx, row) in results.iter().enumerate() {
        println!(
            "{:>4} {:>24} {:>24} {:>10} ",
            idx,
            row.get::<String, &str>("name"),
            row.get::<String, &str>("time"),
            row.get::<String, &str>("language")
        );
    }

    Ok(())
}

pub async fn log_kata(
    pool: &SqlitePool,
    kata_name: String,
    language: Language,
) -> Result<(), Box<dyn Error>> {
    let result = sqlx::query(r#"UPDATE status SET time = datetime() WHERE id = (SELECT id from katas WHERE name = $1 ) AND language = ?2;"#)
        .bind(kata_name.as_str())
        .bind(language.to_string())
        .execute(pool)
        .await?
        .rows_affected();

    println!("Rows updated 1: {}", result);
    if result <= 0 {
        println!("tryng to insert into status");
        dbg!(kata_name.as_str());
        println!("{:?}", "test_kata_1");
        let insert_statement = sqlx::query(
            r#"INSERT into status (id, time, language)
               VALUES ((SELECT id from katas WHERE name = $1 ), datetime("1970-01-01 00:00:00"), $2);"#)
                .bind(kata_name.as_str())
                .bind(language.to_string())
                .execute(pool)
                .await?
                .rows_affected();
        println!("Rows updated 2: {}", insert_statement);
    };

    Ok(())
}
