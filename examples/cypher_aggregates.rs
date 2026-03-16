use velr::{CellRef, Velr};

// This example shows how to use aggregate functions in openCypher.
//
// Query shape:
//   MATCH (p:Person)
//   RETURN count(p), min(p.born), max(p.born), avg(p.born), collect(p.name)
//
// Meaning:
//   match many rows, then compute summary values across those rows.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a few Person nodes we can aggregate over.
    db.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967}),
          (:Person {name:'Laurence Fishburne', born:1961}),
          (:Person {name:'Hugo Weaving', born:1960});
        "#,
    )?;

    // Aggregate functions summarize the matched rows:
    //
    // - count(p)    -> number of matched rows
    // - min(...)    -> smallest value
    // - max(...)    -> largest value
    // - avg(...)    -> average value
    // - collect(...) -> list of values
    //
    // Because this query only returns aggregate expressions,
    // it produces a single result row.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN
          count(p) AS people,
          min(p.born) AS first_born,
          max(p.born) AS last_born,
          avg(p.born) AS average_born,
          collect(p.name) AS names
        "#,
    )?;

    println!("aggregate results:");
    table.for_each_row(|row| {
        let people = match row[0] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        let first_born = match row[1] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        let last_born = match row[2] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        let average_born = match row[3] {
            CellRef::Integer(i) => i.to_string(),
            CellRef::Float(f) => format!("{f:.1}"),
            _ => "<unexpected>".to_string(),
        };

        let names = match row[4] {
            CellRef::Json(bytes) | CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  people: {people}");
        println!("  first born: {first_born}");
        println!("  last born: {last_born}");
        println!("  average born: {average_born}");
        println!("  names: {names}");
        Ok(())
    })?;

    Ok(())
}
