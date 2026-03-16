use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    // Open an in-memory database.
    // Use `Some("path.db")` instead if you want a file-backed database.
    let db = Velr::open(None)?;

    // Seed a few example nodes that we can query.
    db.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967}),
          (:Person {name:'Laurence Fishburne', born:1961});
        "#,
    )?;

    // Execute a query that is expected to return exactly one result table.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name, p.born AS born
        ORDER BY born
        "#,
    )?;

    // Print the column names reported by the result table.
    println!("columns: {:?}", table.column_names());

    // Iterate over each row in the table.
    //
    // Each row is exposed as a slice of `CellRef`, so we match on the
    // expected type in each column.
    table.for_each_row(|row| {
        // Column 0 is `name`, returned as text bytes.
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        // Column 1 is `born`, returned as an integer.
        let born = match row[1] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("name={name}, born={born}");
        Ok(())
    })?;

    Ok(())
}
