use velr::{CellRef, Velr};

// This example shows how a graph can model ticket dependencies.
//
// In this small graph we store:
//
// - tickets
// - people
//
// and connect them with relationships such as:
//
// - (:Ticket)-[:BLOCKS]->(:Ticket)
// - (:Person)-[:ASSIGNED_TO]->(:Ticket)
//
// This lets us ask questions like:
//
// - Which tickets are blocked by other tickets?
// - Who is assigned to the blocking work?
// - What dependency chains exist across a set of tickets?

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small ticket graph.
    //
    // In this example:
    // - TICKET-1 is blocked by TICKET-2
    // - TICKET-2 is blocked by TICKET-3
    // - each ticket is assigned to a person
    db.run(
        r#"
        CREATE
          (frodo:Person {name:'Frodo Baggins'}),
          (sam:Person {name:'Samwise Gamgee'}),
          (gandalf:Person {name:'Gandalf'}),

          (t1:Ticket {
            key:'TICKET-1',
            title:'Ship embedded graph query API',
            status:'Blocked'
          }),
          (t2:Ticket {
            key:'TICKET-2',
            title:'Finish query planner integration',
            status:'In Progress'
          }),
          (t3:Ticket {
            key:'TICKET-3',
            title:'Stabilize runtime ABI',
            status:'Open'
          }),
          (t4:Ticket {
            key:'TICKET-4',
            title:'Write Rust driver examples',
            status:'Open'
          }),

          (frodo)-[:ASSIGNED_TO]->(t1),
          (sam)-[:ASSIGNED_TO]->(t2),
          (gandalf)-[:ASSIGNED_TO]->(t3),
          (frodo)-[:ASSIGNED_TO]->(t4),

          (t2)-[:BLOCKS]->(t1),
          (t3)-[:BLOCKS]->(t2);
        "#,
    )?;

    // Find tickets that are blocked by other tickets.
    //
    // Read:
    //   (blocker:Ticket)-[:BLOCKS]->(blocked:Ticket)
    //
    // as:
    //   "ticket `blocker` blocks ticket `blocked`."
    let mut table = db.exec_one(
        r#"
        MATCH (blocker:Ticket)-[:BLOCKS]->(blocked:Ticket)
        RETURN
          blocked.key AS blocked_ticket,
          blocked.title AS blocked_title,
          blocker.key AS blocker_ticket,
          blocker.title AS blocker_title
        ORDER BY blocked_ticket
        "#,
    )?;

    println!("blocked tickets:");
    table.for_each_row(|row| {
        let blocked_ticket = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let blocked_title = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let blocker_ticket = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let blocker_title = match row[3] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {blocked_ticket} ({blocked_title})");
        println!("    blocked by {blocker_ticket} ({blocker_title})");
        Ok(())
    })?;

    // Find who is assigned to the blocking ticket.
    let mut table = db.exec_one(
        r#"
        MATCH (owner:Person)-[:ASSIGNED_TO]->(blocker:Ticket)-[:BLOCKS]->(blocked:Ticket)
        RETURN
          blocked.key AS blocked_ticket,
          blocker.key AS blocker_ticket,
          owner.name AS blocker_owner
        ORDER BY blocked_ticket
        "#,
    )?;

    println!();
    println!("owners of blocking tickets:");
    table.for_each_row(|row| {
        let blocked_ticket = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let blocker_ticket = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let blocker_owner = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {blocked_ticket} is blocked by {blocker_ticket}, owned by {blocker_owner}");
        Ok(())
    })?;

    // Follow a dependency chain of variable length.
    //
    // Read:
    //   (start:Ticket)<-[:BLOCKS*1..3]-(upstream:Ticket)
    //
    // as:
    //   "find upstream tickets that block `start` through a chain
    //    of between 1 and 3 BLOCKS relationships."
    let mut table = db.exec_one(
        r#"
        MATCH p = (start:Ticket {key:'TICKET-1'})<-[:BLOCKS*1..3]-(upstream:Ticket)
        RETURN
          start.key AS ticket,
          upstream.key AS upstream_ticket,
          length(p) AS dependency_hops
        ORDER BY dependency_hops, upstream_ticket
        "#,
    )?;

    println!();
    println!("dependency chain for TICKET-1:");
    table.for_each_row(|row| {
        let ticket = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let upstream_ticket = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let dependency_hops = match row[2] {
            CellRef::Integer(i) => i,
            _ => -1,
        };

        println!("  {ticket} depends on {upstream_ticket} ({dependency_hops} hop(s) away)");
        Ok(())
    })?;

    Ok(())
}
