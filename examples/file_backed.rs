use std::fs;

use velr::{CellRef, Velr};

const DB_PATH: &str = "file_backed_example.velr";

fn main() -> velr::Result<()> {
    // Start from a clean file so the example is repeatable.
    let _ = fs::remove_file(DB_PATH);

    // Open a file-backed database and write some data.
    {
        let db = Velr::open(Some(DB_PATH))?;

        db.run(
            r#"
            CREATE
              (:Person {name:'Frodo Baggins', role:'Ring-bearer'}),
              (:Person {name:'Samwise Gamgee', role:'Companion'}),
              (:Person {name:'Gandalf', role:'Wizard'});
            "#,
        )?;

        println!("wrote data to {DB_PATH}");
    } // connection is dropped here

    // Reopen the same database file and verify that the data is still there.
    {
        let db = Velr::open(Some(DB_PATH))?;

        let mut table = db.exec_one(
            r#"
            MATCH (p:Person)
            RETURN p.name AS name, p.role AS role
            ORDER BY name
            "#,
        )?;

        println!("reopened {DB_PATH}");
        println!("persisted rows:");

        table.for_each_row(|row| {
            let name = match row[0] {
                CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
                _ => "<unexpected>",
            };

            let role = match row[1] {
                CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
                _ => "<unexpected>",
            };

            println!("  {name} ({role})");
            Ok(())
        })?;
    }

    Ok(())
}
