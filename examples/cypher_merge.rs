// This example shows how to use `MERGE` in openCypher.
//
// Query shape:
//   MERGE (p:Person {name:'Frodo'})
//
// Meaning:
//   find a node that matches the given pattern, or create it if it does not exist.
//
// `MERGE` is useful when you want idempotent writes:
// running the same query multiple times should not create duplicates.

use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Run the same MERGE multiple times.
    //
    // Because the pattern is the same each time, this should leave us with
    // only one matching Person node.
    db.run(
        r#"
        MERGE (:Person {name:'Frodo Baggins'});
        MERGE (:Person {name:'Frodo Baggins'});
        MERGE (:Person {name:'Frodo Baggins'});
        "#,
    )?;

    // Add one more distinct person.
    db.run(
        r#"
        MERGE (:Person {name:'Samwise Gamgee'});
        "#,
    )?;

    // Query the graph back.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name
        ORDER BY name
        "#,
    )?;

    println!("people after MERGE:");
    table.for_each_row(|row| {
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {name}");
        Ok(())
    })?;

    // Show that Frodo exists only once.
    let mut counts = db.exec_one(
        r#"
        MATCH (p:Person {name:'Frodo Baggins'})
        RETURN count(p) AS count
        "#,
    )?;

    counts.for_each_row(|row| {
        let count = match row[0] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!();
        println!("Frodo Baggins node count: {count}");
        Ok(())
    })?;

    Ok(())
}
