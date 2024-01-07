use sqlx::SqliteConnection;

pub async fn setup_tables(conn: &mut SqliteConnection) {
    let result = sqlx::query(
        "CREATE TABLE IF NOT EXISTS 
        katas (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(140) NOT NULL);",
    )
    .execute(conn)
    .await
    .unwrap();
    println!("Result: {:?}", result);
}
