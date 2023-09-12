use rusqlite::Result;
mod db;

#[derive(Debug)]
struct Todos {
    task: String,
    completed: bool,
}

pub fn add(task: String) -> Result<()> {
    let conn = db::conn().unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS todo (
            id INTEGER PRIMARY KEY,
            task TEXT NOT NULL,
            completed BOOLEAN NOT NULL
        )",
        (), // empty list of parameters.
    )?;
    let new_todo = Todos {
        task,
        completed: false,
    };
    conn.execute(
        "INSERT INTO todo (task, completed) VALUES (?1, ?2)",
        (&new_todo.task, &new_todo.completed),
    )?;
    Ok(())
}

pub fn list_todos() -> Result<()> {
    let conn = db::conn().unwrap();
    let mut stmt = conn.prepare("SELECT id, task, completed FROM todo")?;
    let todos_iter = stmt.query_map([], |row| {
        Ok(Todos {
            task: row.get(1)?,
            completed: row.get(2)?,
        })
    })?;

    let mut counter = 1;
    for todo in todos_iter {
        println!("Found todos {}. {}", counter, todo.unwrap().task);
        counter += 1
    }
    Ok(())
}
