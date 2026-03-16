use velr::{CellRef, Velr};

// This example shows how to filter query results with `WHERE`.
//
// Query shape:
//   MATCH (p:Person)
//   WHERE p.born < 1965
//   RETURN p.name, p.born
//
// Meaning:
//   find Person nodes, keep only the ones that match the condition,
//   and return values from the remaining rows.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a few Person nodes to query.
    db.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967}),
          (:Person {name:'Laurence Fishburne', born:1961}),
          (:Person {name:'Hugo Weaving', born:1960});
        "#,
    )?;

    // `MATCH` finds candidate rows.
    // `WHERE` filters those rows.
    //
    // Read:
    //   MATCH (p:Person)
    //   WHERE p.born < 1965
    //
    // as:
    //   "find Person nodes, then keep only the ones born before 1965."
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        WHERE p.born < 1965
        RETURN p.name AS name, p.born AS born
        ORDER BY born, name
        "#,
    )?;

    println!("people born before 1965:");
    table.for_each_row(|row| {
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let born = match row[1] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {name} ({born})");
        Ok(())
    })?;

    Ok(())
}
