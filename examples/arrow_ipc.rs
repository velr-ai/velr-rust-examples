use arrow2::array::{Array, Int64Array, Utf8Array};
use velr::{CellRef, Velr};


// Run with cargo run --example arrow_ipc --features arrow-ipc
fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;

    // Build two Arrow columns in Rust.
    let names = Utf8Array::<i64>::from(vec![Some("Frodo"), Some("Sam"), Some("Gandalf")]);
    let ages = Int64Array::from_slice([50_i64, 38, 2019]);

    let columns = vec!["name".to_string(), "age".to_string()];
    let arrays: Vec<Box<dyn Array>> = vec![names.boxed(), ages.boxed()];

    // Bind the Arrow arrays as a logical table named `_people`.
    db.bind_arrow("_people", columns, arrays)?;

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

    println!("rows returned from Velr:");
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

    // Export the same result table as an Arrow IPC file in memory.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name, p.age AS age
        ORDER BY age
        "#,
    )?;

    let ipc = table.to_arrow_ipc_file()?;
    println!();
    println!("Arrow IPC bytes: {}", ipc.len());

    Ok(())
}
