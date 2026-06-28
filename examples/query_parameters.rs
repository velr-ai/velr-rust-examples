use velr::{CellRef, QueryOptions, Velr};

fn text(cell: CellRef<'_>) -> &str {
    cell.as_str_utf8()
        .and_then(Result::ok)
        .unwrap_or("<unexpected>")
}

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Parameters are bound out of band. The query text uses `$name`, while the
    // Rust API receives parameter names without the leading `$`.
    db.run_with_params(
        "CREATE (:Person {name: $name, role: $role, age: $age})",
        velr::params! {
            name: "Ada Lovelace",
            role: "researcher",
            age: 36_i64,
        }?,
    )?;

    db.run_with_params(
        "CREATE (:Person {name: $name, role: $role, age: $age})",
        velr::params! {
            name: "Grace Hopper",
            role: "engineer",
            age: 85_i64,
        }?,
    )?;

    // Strings stay strings even when they contain characters that would be
    // meaningful in Cypher source text.
    db.run_with_params(
        "CREATE (:Person {name: $name, role: $role, age: $age})",
        velr::params! {
            name: "Alice') MATCH (n) RETURN n //",
            role: "researcher",
            age: 42_i64,
        }?,
    )?;

    let mut table = db.exec_one_with_options(
        r#"
        MATCH (p:Person)
        WHERE p.role = $role AND p.age >= $min_age
        RETURN p.name AS name, p.age AS age
        ORDER BY age, name
        "#,
        QueryOptions::max_result_rows(10)
            .with_param("role", "researcher")?
            .with_param("min_age", 30_i64)?,
    )?;

    println!("researchers aged at least 30:");
    table.for_each_row(|row| {
        let name = text(row[0]);
        let age = match row[1] {
            CellRef::Integer(value) => value,
            _ => -1,
        };
        println!("  {name} ({age})");
        Ok(())
    })?;

    Ok(())
}
