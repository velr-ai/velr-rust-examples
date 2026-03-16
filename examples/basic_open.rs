use velr::Velr;

fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;
    db.run("CREATE (:Example {name:'in-memory'})")?;
    println!("opened in-memory database");

    // Open a file-backed database.
    let path = "basic_open.db";
    let db = Velr::open(Some(path))?;
    db.run("CREATE (:Example {name:'file-backed'})")?;
    println!("opened file-backed database at {path}");

    Ok(())
}
