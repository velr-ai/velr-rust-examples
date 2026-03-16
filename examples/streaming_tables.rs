use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    // Open an in-memory database and seed a little data.
    let db = Velr::open(None)?;

    db.run(
        r#"
        CREATE
          (:Movie {title:'The Matrix', released:1999}),
          (:Movie {title:'Inception', released:2010});
        "#,
    )?;

    // `exec()` can return multiple result tables when the Cypher text
    // contains multiple statements.
    let mut stream = db.exec(
        r#"
        MATCH (m:Movie {title:'The Matrix'})
        RETURN m.title AS title;

        MATCH (m:Movie {title:'Inception'})
        RETURN m.released AS released;
        "#,
    )?;

    let mut table_no = 0;

    // Pull result tables one by one until the stream is exhausted.
    while let Some(mut table) = stream.next_table()? {
        table_no += 1;
        println!("table #{table_no}");
        println!("columns: {:?}", table.column_names());

        // Each table can then be iterated row by row.
        table.for_each_row(|row| {
            for (i, cell) in row.iter().enumerate() {
                match cell {
                    CellRef::Null => println!("  col[{i}] = null"),
                    CellRef::Bool(v) => println!("  col[{i}] = {v}"),
                    CellRef::Integer(v) => println!("  col[{i}] = {v}"),
                    CellRef::Float(v) => println!("  col[{i}] = {v}"),
                    CellRef::Text(bytes) => {
                        println!("  col[{i}] = {}", std::str::from_utf8(bytes).unwrap())
                    }
                    CellRef::Json(bytes) => {
                        println!("  col[{i}] = {}", std::str::from_utf8(bytes).unwrap())
                    }
                }
            }
            Ok(())
        })?;

        println!();
    }

    Ok(())
}
