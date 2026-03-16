use velr::{CellRef, Velr};

// This example shows how to match variable-length paths in openCypher.
//
// Query pattern:
//   MATCH p = (a:Person)-[:KNOWS*1..3]->(b:Person)
//   RETURN a.name, b.name, length(p)
//
// Meaning:
//   starting from one Person node, follow between 1 and 3 outgoing KNOWS
//   relationships to reach another Person node.
//
// `*1..3` means:
// - at least 1 hop
// - at most 3 hops
//
// `length(p)` returns the number of relationships in the matched path.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small graph with a simple chain of KNOWS relationships.
    db.run(
        r#"
        CREATE
          (frodo:Person {name:'Frodo'}),
          (sam:Person {name:'Sam'}),
          (merry:Person {name:'Merry'}),
          (pippin:Person {name:'Pippin'}),
          (gandalf:Person {name:'Gandalf'}),
          (frodo)-[:KNOWS]->(sam),
          (sam)-[:KNOWS]->(merry),
          (merry)-[:KNOWS]->(pippin),
          (pippin)-[:KNOWS]->(gandalf);
        "#,
    )?;

    // Match paths starting from Frodo with between 1 and 3 KNOWS hops.
    let mut table = db.exec_one(
        r#"
        MATCH p = (a:Person {name:'Frodo'})-[:KNOWS*1..3]->(b:Person)
        RETURN a.name AS from, b.name AS to, length(p) AS hops
        ORDER BY hops, to
        "#,
    )?;

    println!("variable-length paths from Frodo:");
    table.for_each_row(|row| {
        let from = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let to = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let hops = match row[2] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {from} -> {to} (hops: {hops})");
        Ok(())
    })?;

    Ok(())
}
