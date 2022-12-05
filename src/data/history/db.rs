use std::str::FromStr;

use chrono::Utc;
use lazy_static::lazy_static;
use sqlx::{
    migrate::MigrateDatabase,
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    ConnectOptions, Executor, SqlitePool, Statement,
};

use crate::data::filfox::models::MinerInfo;

lazy_static! {
    pub static ref HISTORY_DB: String =
        std::env::var("HISTORY_DB").unwrap_or_else(|_| "history.db".to_string());
}

pub async fn init_history_db() -> anyhow::Result<SqlitePool> {
    let conn = if !sqlx::Sqlite::database_exists(&HISTORY_DB).await? {
        sqlx::Sqlite::create_database(&HISTORY_DB).await?;
        let mut options = SqliteConnectOptions::from_str(&HISTORY_DB)
            .unwrap()
            .journal_mode(SqliteJournalMode::Off)
            .synchronous(SqliteSynchronous::Off);

        options.log_statements(log::LevelFilter::Trace);
        let conn = SqlitePoolOptions::new().connect_with(options).await?;

        // conn.execute("PRAGMA locking_mode = EXCLUSIVE;").await?;
        conn.execute(
            "CREATE TABLE history (
            id                      INTEGER PRIMARY KEY AUTOINCREMENT,
            name                    TEXT NOT NULL,
            timestamp               INTEGER NOT NULL,
            pledge                  REAL NOT NULL,
            power                   REAL NOT NULL,
            blocks                  INTEGER NOT NULL,
            rewards                 REAL NOT NULL
        )",
        )
        .await?;

        conn
    } else {
        let mut options = SqliteConnectOptions::from_str(&HISTORY_DB)
            .unwrap()
            .journal_mode(SqliteJournalMode::Off)
            .synchronous(SqliteSynchronous::Off);
        options.log_statements(log::LevelFilter::Trace);
        SqlitePoolOptions::new().connect_with(options).await?
    };

    Ok(conn)
}

pub type DealDbType = (
    String, // 0    name
    i64,    // 1    timestamp
    f64,    // 2    pledge
    f64,    // 3    power
    i64,    // 4    blocks
    f64,    // 5    rewards
);

pub type DealDbTypeFull = (
    i64,
    String, // 0    name
    i64,    // 1    timestamp
    f64,    // 2    pledge
    f64,    // 3    power
    i64,    // 4    blocks
    f64,    // 5    rewards
);

pub async fn insert_db(conn: SqlitePool, data: DealDbType) -> anyhow::Result<()> {
    let mut db = conn.begin().await?;
    let stmt_with_area = conn
        .prepare(
            "
            INSERT INTO history (name,timestamp,pledge,power,blocks,rewards)
            VALUES(?, ?, ?, ?, ?, ?);",
        )
        .await?;

    stmt_with_area
        .query()
        .bind(data.0) // name
        .bind(data.1) // timestamp
        .bind(data.2) // pledge
        .bind(data.3) // power
        .bind(data.4) // blocks
        .bind(data.5) // rewards
        .execute(&mut db)
        .await?;
    db.commit().await?;
    Ok(())
}

// get <name> between time <from> and <to>
pub async fn get_db(
    conn: SqlitePool,
    name: String,
    from: i64,
    to: i64,
) -> anyhow::Result<(Vec<i64>, Vec<MinerInfo>)> {
    let sql = r#"SELECT * from history
    WHERE name=? AND timestamp > ? AND timestamp < ?
    ORDER BY timestamp ASC"#
        .to_string();

    let tmp: Result<Vec<DealDbTypeFull>, _> = sqlx::query_as(&sql)
        .bind(name)
        .bind(from)
        .bind(to)
        .fetch_all(&conn)
        .await;

    match tmp {
        Ok(data) => {
            let mut times = vec![];
            let mut infos = vec![];

            for t in data {
                times.push(t.2);
                let info: MinerInfo = t.into();
                infos.push(info);
            }

            Ok((times, infos))
        }
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}

#[tokio::test]
async fn test_get_db() -> anyhow::Result<()> {
    let db = init_history_db().await?;

    let t = get_db(db, "test".to_string(), 1670219812 - 1, 1670219812 + 1).await?;
    dbg!(t);

    Ok(())
}

#[tokio::test]
async fn test_insert_db() -> anyhow::Result<()> {
    let db = init_history_db().await?;
    let timestamp = Utc::now().timestamp();

    let item: DealDbType = (
        "test".to_string(),
        timestamp + 10000,
        825017.190309711,
        94327.6875,
        88404,
        2066866.8556792436,
    );

    insert_db(db, item).await?;

    Ok(())
}
