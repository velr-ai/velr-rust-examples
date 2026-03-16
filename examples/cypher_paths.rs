use velr::{CellRef, Velr};

// This example shows how to match and return a path in openCypher.
//
// Query pattern:
//   MATCH p = (a:Person)-[:KNOWS]->(b:Person)-[:KNOWS]->(c:Person)
//   RETURN a.name, b.name, c.name, length(p)
//
// Meaning:
//   find a path that starts at one Person node, follows a KNOWS relationship
//   to a second Person node, then follows another KNOWS relationship to a
//   third Person node.
//
// `length(p)` returns the number of relationships in the path.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small graph with a chain of KNOWS relationships.
    db.run(
        r#"
        CREATE
          (frodo:Person {name:'Frodo'}),
          (sam:Person {name:'Sam'}),
          (merry:Person {name:'Merry'}),
          (pippin:Person {name:'Pippin'}),
          (frodo)-[:KNOWS]->(sam),
          (sam)-[:KNOWS]->(merry),
          (merry)-[:KNOWS]->(pippin);
        "#,
    )?;

    // Match paths of exactly two KNOWS hops.
    //
    // Read:
    //   p = (a)-[:KNOWS]->(b)-[:KNOWS]->(c)
    //
    // as:
    //   "bind the whole matched path to `p`, and also bind each node
    //    along the way to `a`, `b`, and `c`."
    let mut table = db.exec_one(
        r#"
        MATCH p = (a:Person)-[:KNOWS]->(b:Person)-[:KNOWS]->(c:Person)
        RETURN
          a.name AS from,
          b.name AS via,
          c.name AS to,
          length(p) AS hops
        ORDER BY from, via, to
        "#,
    )?;

    println!("paths of two KNOWS hops:");
    table.for_each_row(|row| {
        let from = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let via = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let to = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let hops = match row[3] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {from} -> {via} -> {to} (hops: {hops})");
        Ok(())
    })?;

    Ok(())
}
