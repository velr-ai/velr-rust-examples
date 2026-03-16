use velr::{CellRef, Velr};

// This example shows both savepoint styles supported by the Velr Rust driver:
//
// - scoped savepoints via `savepoint()`
// - named savepoints via `savepoint_named(...)`
//
// It also shows the difference between:
// - rolling back a scoped savepoint
// - rolling back to a named savepoint and continuing work

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Start a transaction. All savepoints live inside a transaction.
    let tx = db.begin_tx()?;

    // This write will survive because it happens before the savepoint rollback.
    tx.run("CREATE (:Temp {k:'outer'})")?;

    // Scoped savepoint:
    // if we roll back this savepoint, only the work done after it is undone.
    {
        let sp = tx.savepoint()?;
        tx.run("CREATE (:Temp {k:'inner-scoped'})")?;
        sp.rollback()?;
    }

    // Named savepoint:
    // it remains active until explicitly released or until the transaction ends.
    tx.savepoint_named("before_named")?;
    tx.run("CREATE (:Temp {k:'inner-named'})")?;

    // Roll back to the named savepoint.
    // This removes `inner-named` but keeps the savepoint active.
    tx.rollback_to("before_named")?;

    // Do more work after the rollback.
    tx.run("CREATE (:Temp {k:'after-rollback'})")?;

    // Release the named savepoint once we no longer need it.
    tx.release_savepoint("before_named")?;

    // Commit the remaining work.
    tx.commit()?;

    // Verify what is still in the database.
    let mut table = db.exec_one(
        r#"
        MATCH (n:Temp)
        RETURN n.k AS k
        ORDER BY k
        "#,
    )?;

    println!("rows after commit:");
    table.for_each_row(|row| {
        if let CellRef::Text(bytes) = row[0] {
            println!("  {}", std::str::from_utf8(bytes).unwrap());
        }
        Ok(())
    })?;

    Ok(())
}
