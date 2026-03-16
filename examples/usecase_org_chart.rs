use velr::{CellRef, Velr};

// This example shows how a graph can model an organization chart.
//
// In this small graph we store:
//
// - people
// - teams
//
// and connect them with relationships such as:
//
// - (:Person)-[:REPORTS_TO]->(:Person)
// - (:Person)-[:MEMBER_OF]->(:Team)
// - (:Person)-[:MANAGES]->(:Team)
//
// This lets us ask questions like:
//
// - Who reports to whom?
// - Which team is a person part of?
// - Who manages a team?
// - What management chain exists above a person?
//
// Graphs are useful here because hierarchies and reporting lines are
// naturally represented as connected relationships.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small org chart.
    //
    // In this example:
    // - Frodo and Sam are in Engineering
    // - Merry is in Operations
    // - Aragorn manages Engineering
    // - Gandalf manages Operations
    // - Frodo and Sam report to Aragorn
    // - Merry reports to Gandalf
    // - Aragorn and Gandalf report to Elrond
    db.run(
        r#"
        CREATE
          (frodo:Person {name:'Frodo Baggins', title:'Engineer'}),
          (sam:Person {name:'Samwise Gamgee', title:'Engineer'}),
          (merry:Person {name:'Meriadoc Brandybuck', title:'Operations Specialist'}),
          (aragorn:Person {name:'Aragorn', title:'Engineering Manager'}),
          (gandalf:Person {name:'Gandalf', title:'Operations Manager'}),
          (elrond:Person {name:'Elrond', title:'Director'}),

          (engineering:Team {name:'Engineering'}),
          (operations:Team {name:'Operations'}),

          (frodo)-[:MEMBER_OF]->(engineering),
          (sam)-[:MEMBER_OF]->(engineering),
          (merry)-[:MEMBER_OF]->(operations),
          (aragorn)-[:MEMBER_OF]->(engineering),
          (gandalf)-[:MEMBER_OF]->(operations),

          (aragorn)-[:MANAGES]->(engineering),
          (gandalf)-[:MANAGES]->(operations),

          (frodo)-[:REPORTS_TO]->(aragorn),
          (sam)-[:REPORTS_TO]->(aragorn),
          (merry)-[:REPORTS_TO]->(gandalf),
          (aragorn)-[:REPORTS_TO]->(elrond),
          (gandalf)-[:REPORTS_TO]->(elrond);
        "#,
    )?;

    // Show team membership together with the team manager.
    let mut table = db.exec_one(
        r#"
        MATCH (p:Person)-[:MEMBER_OF]->(t:Team)<-[:MANAGES]-(manager:Person)
        RETURN
          p.name AS person,
          t.name AS team,
          manager.name AS manager
        ORDER BY team, person
        "#,
    )?;

    println!("people, their team, and team manager:");
    table.for_each_row(|row| {
        let person = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let team = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let manager = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {person} -> {team} (managed by {manager})");
        Ok(())
    })?;

    // Show direct reporting lines.
    let mut table = db.exec_one(
        r#"
        MATCH (employee:Person)-[:REPORTS_TO]->(manager:Person)
        RETURN
          employee.name AS employee,
          manager.name AS manager
        ORDER BY manager, employee
        "#,
    )?;

    println!();
    println!("direct reporting lines:");
    table.for_each_row(|row| {
        let employee = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let manager = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {employee} reports to {manager}");
        Ok(())
    })?;

    // Follow the management chain above Frodo.
    //
    // Read:
    //   (start:Person)-[:REPORTS_TO*1..3]->(manager:Person)
    //
    // as:
    //   "starting from Frodo, follow between 1 and 3 REPORTS_TO
    //    relationships upward through the org chart."
    let mut table = db.exec_one(
        r#"
        MATCH p = (start:Person {name:'Frodo Baggins'})-[:REPORTS_TO*1..3]->(manager:Person)
        RETURN
          start.name AS employee,
          manager.name AS manager,
          length(p) AS levels_up
        ORDER BY levels_up, manager
        "#,
    )?;

    println!();
    println!("management chain above Frodo:");
    table.for_each_row(|row| {
        let employee = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let manager = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let levels_up = match row[2] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {employee} -> {manager} ({levels_up} level(s) up)");
        Ok(())
    })?;

    Ok(())
}
