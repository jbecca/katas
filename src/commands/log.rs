use std::error::Error;

use clap::Parser;
use lib_katas::util::{parse_config, Difficulty, Language, self};
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[derive(Parser, Debug)]
pub(crate) struct LogArgs {
    /// name of kata to update
    #[arg(short, long)]
    name: String,

    /// language used for the kata
    #[arg(short, long, value_enum)]
    language: Language,

    /// difficulty for practice attempt
    #[arg(short, long, value_enum)]
    difficulty: Difficulty,
}

/// This function and SQLite table are separate from the 
/// spaced repetition algorithm logic. This also needs to be
/// done in each kata attempt. The purpose of this table
/// and function is to have a record of every attempt and when.
/// This can be used for some metrics and analysis the user
/// may be interested in
async fn log_kata(
    pool: &SqlitePool,
    kata_name: String,
    difficulty: Difficulty,
) -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::log::log_kata");
    println!("trying to insert into status");
    dbg!(kata_name.as_str());
    println!("{:?}", "test_kata_1");
    let insert_statement = sqlx::query(
        r#"INSERT into attempts (id, difficulty, time)
            VALUES (
            (SELECT id from katas
                WHERE name = $1 ),
            $2,
            datetime());"#,
    )
    .bind(kata_name.as_str())
    .bind(difficulty.to_string())
    .execute(pool)
    .await?
    .rows_affected();
    println!("Rows added: {}", insert_statement);

    Ok(())
}

/// This function handles querying and updating the database using the
/// SM2 algorithm.
async fn update_status(pool: &SqlitePool, kata_name: String, difficulty: Difficulty) -> Result<(), Box<dyn Error>> {
    trace!("Getting entry in status table for current values");
    let query = sqlx::query(
        r#"SELECT * from status
        INNER JOIN katas on katas.id = status.id
        WHERE name = $1
        ORDER BY due
        ASC LIMIT 1"#
        //AND due < datetime()
    ).bind(kata_name.as_str())
    .fetch_one(pool)
    .await?;

    trace!("getting values we need from query");
    let rep_num = query.get::<i32, &str>("n_success");
    trace!("Original rep num {:?}", &rep_num);
    let ef = query.get::<f32, &str>("easiness_factor");
    trace!("Original EF {:?}", &ef);
    let last_interval = query.get::<i32, &str>("last_interval");
    trace!("Original interval {:?}", &last_interval);

    let (new_rep_num, new_ef, new_interval) = util::sm2_algo(difficulty, rep_num, ef, last_interval);
    trace!("New rep num {:?}", &new_rep_num);
    trace!("New EF {:?}", &new_ef);
    trace!("New interval {:?}", &new_interval);

    let new_query = sqlx::query(
        r#"UPDATE status
        SET n_success = ?1,
            easiness_factor = ?2,
            last_interval = ?3,
            due = datetime('now', concat('+', $4, ' day'))"#
    ).bind(new_rep_num)
    .bind(new_ef)
    .bind(new_interval.clone())
    .bind(new_interval)
    .execute(pool)
    .await?;
    debug!("update query: {:?}", new_query);


    Ok(())
}

pub(crate) async fn run(options: LogArgs) -> Result<(), Box<dyn Error>> {
    trace!("Starting commands::log::run");
    let user_cfg = parse_config()?;
    if let Some(loc) = user_cfg["db_location"].as_str() {
        let pool = SqlitePool::connect(&format!("sqlite://{loc}")).await?;
        log_kata(&pool, options.name.clone(), options.difficulty.clone()).await?;
        update_status(&pool, options.name, options.difficulty).await?;
        pool.close().await;
        Ok(())
    } else {
        Err("key db_location not found in TOML file".into())
    }
}
