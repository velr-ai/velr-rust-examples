use velr::Velr;

fn main() -> velr::Result<()> {
    // Open an in-memory database.
    let db = Velr::open(None)?;

    // Build an explain trace for a Cypher query.
    //
    // `explain()` plans the query and returns structured trace data,
    // but does not execute the query itself.
    let trace = db.explain(
        r#"
        MATCH (n) RETURN n
        "#,
    )?;

    // Print how many top-level plans are present in the trace.
    println!("plans: {}", trace.plan_count()?);

    // Render the trace in its compact human-readable form.
    println!();
    println!("{}", trace.to_compact_string()?);

    Ok(())
}
