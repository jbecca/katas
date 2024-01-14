use sqlx::SqliteConnection;

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
    let result = sqlx::query(
        r#"INSERT into katas (name) VALUES ( ?1 );"#)
    .bind(kata_name.as_str())
    .execute(conn)
    .await
    .unwrap();

    println!("Result: {:?}", result);
}
