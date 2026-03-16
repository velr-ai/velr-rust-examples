use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Start a transaction, write some data, and then let the transaction
    // go out of scope without calling `commit()`.
    {
        let tx = db.begin_tx()?;

        tx.run(
            r#"
            CREATE
              (:Person {name:'Frodo Baggins'}),
              (:Person {name:'Samwise Gamgee'});
            "#,
        )?;

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

        println!("dropping transaction without commit...");
    } // transaction is dropped here, so it is rolled back

    // Because the transaction was not committed, the database is unchanged.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)
        RETURN count(p) AS people
        "#,
    )?;

    table.for_each_row(|row| {
        if let CellRef::Integer(n) = row[0] {
            println!("rows visible after transaction drop: {n}");
        }
        Ok(())
    })?;

    Ok(())
}
