use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;

    // Start a transaction.
    let tx = db.begin_tx()?;

    // All writes below are part of the same transaction and are not
    // permanently visible until we commit.
    tx.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967}),
          (:Person {name:'Laurence Fishburne', born:1961});
        "#,
    )?;

    // Queries can also be executed inside the transaction.
    let mut table = tx.exec_one(
        r#"
        MATCH (p:Person)
        RETURN count(p) AS people
        "#,
    )?;

    table.for_each_row(|row| {
        if let CellRef::Integer(n) = row[0] {
            println!("rows visible inside transaction: {n}");
        }
        Ok(())
    })?;

    // Commit makes the transaction durable.
    tx.commit()?;

    // After commit, the data is visible from the connection as usual.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN p.name AS name
        ORDER BY name
        "#,
    )?;

    println!("people after commit:");
    table.for_each_row(|row| {
        if let CellRef::Text(bytes) = row[0] {
            println!("  {}", std::str::from_utf8(bytes).unwrap());
        }
        Ok(())
    })?;

    Ok(())
}
