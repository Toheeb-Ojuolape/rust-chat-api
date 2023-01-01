use rusqlite::{Connection, Result};

fn init_db() -> Result<()> {
    let conn = Connection::open("chat-app.db")?;

    conn.execute(
        "CREATE TABLE messages (
            id INTEGER PRIMARY KEY,
            sender TEXT NOT NULL,
            content TEXT NOT NULL
        )",
        rusqlite::NO_PARAMS,
    )?;

    Ok(())
}