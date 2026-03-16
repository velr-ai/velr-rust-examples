use velr::{CellRef, Velr};

// This example shows how a knowledge graph models entities and the
// relationships between them.
//
// In this small graph we store:
//
// - people
// - companies
// - technologies
//
// and connect them with relationships such as:
//
// - (:Person)-[:WORKS_AT]->(:Company)
// - (:Company)-[:USES]->(:Technology)
// - (:Person)-[:KNOWS_ABOUT]->(:Technology)
//
// This lets us ask connected questions such as:
// "Which technologies is each person exposed to through their company?"

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small knowledge graph.
    //
    // The graph contains:
    // - people
    // - companies
    // - technologies
    // - relationships between them
    db.run(
        r#"
        CREATE
          (frodo:Person {name:'Frodo Baggins'}),
          (sam:Person {name:'Samwise Gamgee'}),
          (gandalf:Person {name:'Gandalf'}),
          (velr:Company {name:'Velr'}),
          (shire_it:Company {name:'Shire IT'}),
          (rust:Technology {name:'Rust'}),
          (sqlite:Technology {name:'SQLite'}),
          (graphs:Technology {name:'Graph Databases'}),
          (agents:Technology {name:'AI Agents'}),

          (frodo)-[:WORKS_AT]->(velr),
          (sam)-[:WORKS_AT]->(shire_it),
          (gandalf)-[:WORKS_AT]->(velr),

          (velr)-[:USES]->(rust),
          (velr)-[:USES]->(sqlite),
          (velr)-[:USES]->(graphs),
          (shire_it)-[:USES]->(sqlite),

          (frodo)-[:KNOWS_ABOUT]->(graphs),
          (sam)-[:KNOWS_ABOUT]->(sqlite),
          (gandalf)-[:KNOWS_ABOUT]->(rust),
          (gandalf)-[:KNOWS_ABOUT]->(agents);
        "#,
    )?;

    // Traverse the graph across multiple entity types:
    //
    //   (p:Person)-[:WORKS_AT]->(c:Company)-[:USES]->(t:Technology)
    //
    // Read this as:
    // "find a person `p`, the company `c` they work at, and the
    // technology `t` that company uses."
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)-[:WORKS_AT]->(c:Company)-[:USES]->(t:Technology)
        RETURN
          p.name AS person,
          c.name AS company,
          t.name AS technology
        ORDER BY person, technology
        "#,
    )?;

    println!("technologies connected to people through their company:");
    table.for_each_row(|row| {
        let person = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let company = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let technology = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {person} -> {company} -> {technology}");
        Ok(())
    })?;

    // A knowledge graph is also useful for finding who knows about
    // a technology that a company uses.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)-[:KNOWS_ABOUT]->(t:Technology)<-[:USES]-(c:Company)
        RETURN
          p.name AS person,
          t.name AS technology,
          c.name AS company
        ORDER BY company, technology, person
        "#,
    )?;

    println!();
    println!("people who know about technologies used by a company:");
    table.for_each_row(|row| {
        let person = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let technology = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        let company = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {person} knows about {technology} used by {company}");
        Ok(())
    })?;

    Ok(())
}
