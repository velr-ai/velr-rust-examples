use velr::{CellRef, Velr};

// This example shows the most basic openCypher query shape:
// matching nodes and returning values from them.
//
// Query pattern:
//   MATCH (p:Person)
//   RETURN p.name, p.born
//
// Meaning:
//   find all nodes with the label `Person` and return selected properties.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a few Person nodes we can query.
    //
    // In openCypher:
    // - `(n:Label)` creates a node with a label
    // - `{key:value}` sets properties on the node
    db.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967}),
          (:Person {name:'Laurence Fishburne', born:1961});
        "#,
    )?;

    // Match all Person nodes and return two properties from each one.
    //
    // Read:
    //   MATCH (p:Person)
    // as:
    //   "find every node `p` with the label Person"
    //
    // Then:
    //   RETURN p.name AS name, p.born AS born
    // selects which values to return for each match.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name, p.born AS born
        ORDER BY born
        "#,
    )?;

    println!("people:");
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
