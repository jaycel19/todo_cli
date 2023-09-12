use rusqlite::{Connection, Result};

pub fn conn() -> Result<Connection> {
    let conn = Connection::open("todos.db")?;
    Ok(conn)
}
