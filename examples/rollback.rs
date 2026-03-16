use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;

    // Start a transaction.
    let tx = db.begin_tx()?;

    // These writes are only visible inside the transaction until it is committed.
    tx.run(
        r#"
        CREATE
          (:Person {name:'Keanu Reeves', born:1964}),
          (:Person {name:'Carrie-Anne Moss', born:1967});
        "#,
    )?;

    // Inside the transaction, the rows are visible.
    let mut table = tx.exec_one(
        r#"
        MATCH (p:Person)
        RETURN count(p) AS people
        "#,
    )?;

    table.for_each_row(|row| {
        if let CellRef::Integer(n) = row[0] {
            println!("rows visible inside transaction before rollback: {n}");
        }
        Ok(())
    })?;

    // Roll back the transaction, discarding all writes done in it.
    tx.rollback()?;

    // After rollback, the database is unchanged.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN count(p) AS people
        "#,
    )?;

    table.for_each_row(|row| {
        if let CellRef::Integer(n) = row[0] {
            println!("rows visible after rollback: {n}");
        }
        Ok(())
    })?;

    Ok(())
}
