use velr::{CellRef, Velr};

// This example shows how to use `WITH` together with aggregate functions
// in openCypher.
//
// Query shape:
//   MATCH (p:Person)-[:ACTED_IN]->(m:Movie)
//   WITH m.title AS movie, count(p) AS actors
//   WHERE actors >= 2
//   RETURN movie, actors
//
// Meaning:
//   first group rows by movie and count how many actors each movie has,
//   then pass the grouped result forward with `WITH`,
//   then filter the grouped rows with `WHERE`.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small movie graph.
    db.run(
        r#"
        CREATE
          (keanu:Person {name:'Keanu Reeves'}),
          (carrie:Person {name:'Carrie-Anne Moss'}),
          (laurence:Person {name:'Laurence Fishburne'}),
          (hugo:Person {name:'Hugo Weaving'}),
          (matrix:Movie {title:'The Matrix'}),
          (john_wick:Movie {title:'John Wick'}),
          (keanu)-[:ACTED_IN]->(matrix),
          (carrie)-[:ACTED_IN]->(matrix),
          (laurence)-[:ACTED_IN]->(matrix),
          (hugo)-[:ACTED_IN]->(matrix),
          (keanu)-[:ACTED_IN]->(john_wick);
        "#,
    )?;

    // `MATCH` produces one row per (person, movie) pair.
    //
    // `WITH m.title AS movie, count(p) AS actors`
    // groups those rows by movie title and counts how many actors each movie has.
    //
    // After that, `WHERE actors >= 2`
    // filters the grouped rows, not the original match rows.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)-[:ACTED_IN]->(m:Movie)
        WITH m.title AS movie, count(p) AS actors
        WHERE actors >= 2
        RETURN movie, actors
        ORDER BY actors DESC, movie
        "#,
    )?;

    println!("movies with at least two actors:");
    table.for_each_row(|row| {
        let movie = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let actors = match row[1] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {movie} ({actors} actors)");
        Ok(())
    })?;

    Ok(())
}
