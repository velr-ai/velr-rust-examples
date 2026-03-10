mod bacth_import;
use velr::{CellRef, Result, Velr};

fn main() -> Result<()> {
    // Open in-memory DB (pass Some("path.db") for file-backed)
    let db = Velr::open(None)?;

    db.run("CREATE (:Person {name:'Keanu Reeves', born:1964})")?;

    let mut t = db.exec_one("MATCH (p:Person) RETURN p.name AS name, p.born AS born")?;

    println!("{:?}", t.column_names());

    t.for_each_row(|row| {
        match row[0] {
            CellRef::Text(bytes) => println!("name={}", std::str::from_utf8(bytes).unwrap()),
            _ => {}
        }
        match row[1] {
            CellRef::Integer(i) => println!("born={i}"),
            _ => {}
        }
        Ok(())
    })
    .unwrap();

    Ok(())
}

#[test]
fn main2() -> Result<()> {
    let db = Velr::open(None)?;
    let mut stream = db.exec(
        "MATCH (m:Movie {title:'The Matrix'}) RETURN m.title AS title;
     MATCH (m:Movie {title:'Inception'})  RETURN m.released AS year",
    )?;

    while let Some(mut table) = stream.next_table()? {
        println!("{:?}", table.column_names());
        table
            .for_each_row(|row| {
                println!("{row:?}");
                Ok(())
            })
            .unwrap();
    }
    Ok(())
}

#[test]
fn arrow_example() -> Result<()> {
    use arrow2::array::{Array, Utf8Array};

    let db = Velr::open(None)?;

    let cols = vec!["name".to_string()];
    let arrays: Vec<Box<dyn Array>> =
        vec![Utf8Array::<i64>::from(vec![Some("Alice"), Some("Bob")]).boxed()];

    db.bind_arrow("_people", cols, arrays)?;
    db.run("UNWIND BIND('_people') AS r CREATE (:Person {name:r.name})")?;

    let mut t = db.exec_one("MATCH (p:Person) RETURN p.name AS name ORDER BY name")?;
    let ipc = t.to_arrow_ipc_file()?;

    println!("IPC bytes: {}", ipc.len());
    Ok(())
}
