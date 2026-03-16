use velr::{CellRef, Velr};

// This example shows how to use `MERGE` for both nodes and relationships.
//
// Query shape:
//   MERGE (a:Person {name:'Frodo'})
//   MERGE (b:Person {name:'Sam'})
//   MERGE (a)-[:KNOWS]->(b)
//
// Meaning:
//   find the matching nodes and relationship, or create them if they do not exist.
//
// This is useful for idempotent graph writes:
// running the same query multiple times should not create duplicate nodes
// or duplicate relationships.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Run the same MERGE pattern multiple times.
    //
    // The graph should still end up with:
    // - one Frodo node
    // - one Sam node
    // - one KNOWS relationship from Frodo to Sam
    db.run(
        r#"
        MERGE (a:Person {name:'Frodo Baggins'})
        MERGE (b:Person {name:'Samwise Gamgee'})
        MERGE (a)-[:KNOWS]->(b);

        MERGE (a:Person {name:'Frodo Baggins'})
        MERGE (b:Person {name:'Samwise Gamgee'})
        MERGE (a)-[:KNOWS]->(b);

        MERGE (a:Person {name:'Frodo Baggins'})
        MERGE (b:Person {name:'Samwise Gamgee'})
        MERGE (a)-[:KNOWS]->(b);
        "#,
    )?;

    // Add another relationship to show the graph can still grow normally.
    db.run(
        r#"
        MERGE (a:Person {name:'Frodo Baggins'})
        MERGE (b:Person {name:'Gandalf'})
        MERGE (a)-[:KNOWS]->(b);
        "#,
    )?;

    // Query the relationships back.
    let mut table = db.exec_one(
        r#"
        MATCH (a:Person)-[:KNOWS]->(b:Person)
        RETURN a.name AS from, b.name AS to
        ORDER BY from, to
        "#,
    )?;

    println!("relationships after MERGE:");
    table.for_each_row(|row| {
        let from = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let to = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {from} -> {to}");
        Ok(())
    })?;

    // Show that the Frodo -> Sam relationship exists only once.
    let mut counts = db.exec_one(
        r#"
        MATCH (:Person {name:'Frodo Baggins'})-[r:KNOWS]->(:Person {name:'Samwise Gamgee'})
        RETURN count(r) AS count
        "#,
    )?;

    counts.for_each_row(|row| {
        let count = match row[0] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!();
        println!("Frodo -> Sam relationship count: {count}");
        Ok(())
    })?;

    Ok(())
}
