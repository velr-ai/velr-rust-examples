use arrow2::array::{Array, Int64Array, Utf8Array};
use velr::{CellRef, Velr};

// Run cargo run --example arrow_chunks --features arrow-ipc
fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;

    // Build chunked Arrow input for two columns.
    //
    // Each inner Vec is the list of chunks for one column.
    // All columns must have the same total row count across their chunks.
    let name_chunks: Vec<Box<dyn Array>> = vec![
        Utf8Array::<i64>::from(vec![Some("Frodo"), Some("Sam")]).boxed(),
        Utf8Array::<i64>::from(vec![Some("Merry"), Some("Pippin")]).boxed(),
    ];

    let age_chunks: Vec<Box<dyn Array>> = vec![
        Int64Array::from_slice([50_i64, 38]).boxed(),
        Int64Array::from_slice([36_i64, 28]).boxed(),
    ];

    let columns = vec!["name".to_string(), "age".to_string()];
    let chunks_per_col: Vec<Vec<Box<dyn Array>>> = vec![name_chunks, age_chunks];

    // Bind the chunked Arrow arrays as a logical table named `_people`.
    db.bind_arrow_chunks("_people", columns, chunks_per_col)?;

    // Import the bound rows into the graph.
    db.run(
        r#"
        UNWIND BIND('_people') AS r
        CREATE (:Person {
            name: r.name,
            age: r.age
        })
        "#,
    )?;

    // Query the imported data back from Velr.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name, p.age AS age
        ORDER BY age
        "#,
    )?;

    println!("rows imported from chunked Arrow input:");
    table.for_each_row(|row| {
        let name = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let age = match row[1] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {name} ({age})");
        Ok(())
    })?;

    Ok(())
}
