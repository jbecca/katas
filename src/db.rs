use sqlx::{SqliteConnection, Row};

pub async fn setup_tables(conn: &mut SqliteConnection) {
    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS 
        katas (id INTEGER PRIMARY KEY AUTOINCREMENT, name VARCHAR(140) NOT NULL);",
    )
    .execute(conn)
    .await
    .unwrap();
    println!("Result: {:?}", result);
}

pub async fn insert_kata_name(conn: &mut SqliteConnection, kata_name: String) {
    let result = sqlx::query(r#"INSERT into katas (name) VALUES ( ?1 );"#)
        .bind(kata_name.as_str())
        .execute(conn)
        .await
        .unwrap();

    println!("Result: {:?}", result);
}

pub async fn list_n_katas(conn: &mut SqliteConnection, number: &u32) {
    let results = sqlx::query(r#"SELECT * from katas LIMIT ?1;"#)
        .bind(number.to_string())
        .fetch_all(conn)
        .await
        .unwrap();

    for (idx, row) in results.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }
}
