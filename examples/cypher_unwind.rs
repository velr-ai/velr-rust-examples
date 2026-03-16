use velr::{CellRef, Velr};

// This example shows how to use `UNWIND` in openCypher.
//
// Query shape:
//   UNWIND ['Frodo', 'Sam', 'Gandalf'] AS name
//   RETURN name
//
// Meaning:
//   take a list value, turn each element into its own row,
//   and then continue the query with one row per element.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // `UNWIND` turns each element in a list into a separate row.
    //
    // Here:
    // - the list is ['Frodo', 'Sam', 'Gandalf']
    // - each element is bound to the variable `name`
    // - one Person node is created per row
    db.run(
        r#"
        UNWIND ['Frodo', 'Sam', 'Gandalf'] AS name
        CREATE (:Person {name: name})
        "#,
    )?;

    // Query the created nodes back to verify the result.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name
        ORDER BY name
        "#,
    )?;

    println!("people created with UNWIND:");
    table.for_each_row(|row| {
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {name}");
        Ok(())
    })?;

    Ok(())
}
