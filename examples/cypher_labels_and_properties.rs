use velr::{CellRef, Velr};

// This example shows how to inspect labels and properties in openCypher.
//
// Functions used:
//   labels(n)      -> list of labels on a node
//   keys(n)        -> list of property names on a node
//   properties(n)  -> map of all properties on a node
//
// Meaning:
//   instead of only reading one property like `p.name`, these functions let
//   you inspect the shape of a node itself.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small graph with labeled nodes and a few properties.
    db.run(
        r#"
        CREATE
          (:Person:Actor {name:'Keanu Reeves', born:1964}),
          (:Person:Actor {name:'Carrie-Anne Moss', born:1967}),
          (:Movie:ScienceFiction {title:'The Matrix', released:1999});
        "#,
    )?;

    // Match Person nodes and inspect:
    // - their labels
    // - their property keys
    // - their full property map
    //
    // `labels(...)`, `keys(...)`, and `properties(...)` typically come back
    // as JSON-like values, so we handle both `Text` and `Json`.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN
          p.name AS name,
          labels(p) AS labels,
          keys(p) AS keys,
          properties(p) AS props
        ORDER BY name
        "#,
    )?;

    println!("person nodes:");
    table.for_each_row(|row| {
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let labels = match row[1] {
            CellRef::Json(bytes) | CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let keys = match row[2] {
            CellRef::Json(bytes) | CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let props = match row[3] {
            CellRef::Json(bytes) | CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  name:   {name}");
        println!("  labels: {labels}");
        println!("  keys:   {keys}");
        println!("  props:  {props}");
        println!();
        Ok(())
    })?;

    Ok(())
}
