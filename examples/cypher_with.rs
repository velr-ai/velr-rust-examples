// This example shows how to use `WITH` in openCypher.
//
// Query shape:
//   MATCH (p:Person)
//   WITH p.name AS name, p.born AS born
//   WHERE born < 1965
//   RETURN name, born
//
// Meaning:
//   `WITH` passes values from one part of the query to the next.
//
// It is commonly used to:
// - rename values
// - keep only the variables you want to continue with
// - apply filtering after an earlier step
// - break a query into clear stages

use velr::{CellRef, Velr};

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

    // `MATCH` finds Person nodes.
    //
    // `WITH` then passes only two projected values forward:
    // - `p.name` as `name`
    // - `p.born` as `born`
    //
    // After that, `WHERE born < 1965` filters the rows that came out of `WITH`.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        WITH p.name AS name, p.born AS born
        WHERE born < 1965
        RETURN name, born
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
