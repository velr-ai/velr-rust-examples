use velr::{CellRef, Velr};

// This example shows one of the core ideas in openCypher:
// matching relationships between nodes.
//
// Query pattern:
//   (p:Person)-[:ACTED_IN]->(m:Movie)
//
// Meaning:
//   find Person nodes connected by an outgoing ACTED_IN relationship
//   to Movie nodes.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small movie graph with:
    // - Person nodes
    // - Movie nodes
    // - ACTED_IN relationships from people to movies
    //
    // In openCypher:
    // - `(n:Label)` is a node with a label
    // - `-[:TYPE]->` is a directed relationship
    db.run(
        r#"
        CREATE
          (keanu:Person {name:'Keanu Reeves'}),
          (carrie:Person {name:'Carrie-Anne Moss'}),
          (laurence:Person {name:'Laurence Fishburne'}),
          (matrix:Movie {title:'The Matrix', released:1999}),
          (john_wick:Movie {title:'John Wick', released:2014}),
          (keanu)-[:ACTED_IN]->(matrix),
          (carrie)-[:ACTED_IN]->(matrix),
          (laurence)-[:ACTED_IN]->(matrix),
          (keanu)-[:ACTED_IN]->(john_wick);
        "#,
    )?;

    // Match the pattern:
    //
    //   (p:Person)-[:ACTED_IN]->(m:Movie)
    //
    // Read it as:
    // "find a Person node `p` that has an outgoing ACTED_IN relationship
    //  to a Movie node `m`."
    //
    // Then return the actor name and movie title for each match.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)-[:ACTED_IN]->(m:Movie)
        RETURN p.name AS actor, m.title AS movie
        ORDER BY actor, movie
        "#,
    )?;

    println!("actors and their movies:");
    table.for_each_row(|row| {
        let actor = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let movie = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {actor} -> {movie}");
        Ok(())
    })?;

    Ok(())
}
