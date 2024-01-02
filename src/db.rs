use sqlx::{Connection, SqliteConnection};

pub fn test() {
    println!("Hello from test function");
}

pub fn insert() {
    // insert a new kata into the local database
    ()
}

pub fn update() {
    // update entry in db;
    ()
}

pub fn delete() {
    // remove entry from database
    ()
}

pub fn read() {
    // read entry from database
    ()
}

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
