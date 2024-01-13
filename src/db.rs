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
        &format!("INSERT into katas VALUES {kata_name}")
    )
    .execute(conn)
    .await
    .unwrap();

    println!("Result: {:?}", result);
}
