use velr::{CellRef, Velr};

// This example shows how a graph can help with simple fraud detection.
//
// In this small graph we store:
//
// - accounts
// - devices
// - IP addresses
// - payment cards
//
// and connect them with relationships such as:
//
// - (:Account)-[:USED_DEVICE]->(:Device)
// - (:Account)-[:LOGGED_IN_FROM]->(:IP)
// - (:Account)-[:USED_CARD]->(:Card)
//
// This lets us ask questions like:
//
// - Which accounts share the same device?
// - Which accounts share the same IP address?
// - Which accounts are connected through shared infrastructure?
//
// Graphs are useful here because suspicious behavior often appears
// as shared connections between entities, not just as isolated rows.

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    // Create a small fraud-like graph.
    //
    // In this example:
    // - acc-1 and acc-2 share the same device and IP
    // - acc-2 and acc-3 share the same payment card
    db.run(
        r#"
        CREATE
          (a1:Account {id:'acc-1', owner:'Frodo'}),
          (a2:Account {id:'acc-2', owner:'Sam'}),
          (a3:Account {id:'acc-3', owner:'Gollum'}),

          (d1:Device {id:'dev-1'}),
          (d2:Device {id:'dev-2'}),

          (ip1:IP {address:'203.0.113.10'}),
          (ip2:IP {address:'198.51.100.20'}),

          (c1:Card {id:'card-1'}),
          (c2:Card {id:'card-2'}),

          (a1)-[:USED_DEVICE]->(d1),
          (a2)-[:USED_DEVICE]->(d1),
          (a3)-[:USED_DEVICE]->(d2),

          (a1)-[:LOGGED_IN_FROM]->(ip1),
          (a2)-[:LOGGED_IN_FROM]->(ip1),
          (a3)-[:LOGGED_IN_FROM]->(ip2),

          (a1)-[:USED_CARD]->(c1),
          (a2)-[:USED_CARD]->(c2),
          (a3)-[:USED_CARD]->(c2);
        "#,
    )?;

    // Find pairs of accounts that share the same device.
    //
    // Read:
    //   (a1)-[:USED_DEVICE]->(d)<-[:USED_DEVICE]-(a2)
    //
    // as:
    //   "two accounts connected to the same device."
    let mut table = db.exec_one(
        r#"
        MATCH (a1:Account)-[:USED_DEVICE]->(d:Device)<-[:USED_DEVICE]-(a2:Account)
        WHERE a1.id < a2.id
        RETURN a1.id AS account_1, a2.id AS account_2, d.id AS shared_device
        ORDER BY account_1, account_2
        "#,
    )?;

    println!("accounts sharing a device:");
    table.for_each_row(|row| {
        let account_1 = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let account_2 = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let shared_device = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {account_1} <-> {account_2} via {shared_device}");
        Ok(())
    })?;

    // Find pairs of accounts that share the same IP address.
    let mut table = db.exec_one(
        r#"
        MATCH (a1:Account)-[:LOGGED_IN_FROM]->(ip:IP)<-[:LOGGED_IN_FROM]-(a2:Account)
        WHERE a1.id < a2.id
        RETURN a1.id AS account_1, a2.id AS account_2, ip.address AS shared_ip
        ORDER BY account_1, account_2
        "#,
    )?;

    println!();
    println!("accounts sharing an IP address:");
    table.for_each_row(|row| {
        let account_1 = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let account_2 = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let shared_ip = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {account_1} <-> {account_2} via {shared_ip}");
        Ok(())
    })?;

    // Find pairs of accounts that share the same payment card.
    let mut table = db.exec_one(
        r#"
        MATCH (a1:Account)-[:USED_CARD]->(c:Card)<-[:USED_CARD]-(a2:Account)
        WHERE a1.id < a2.id
        RETURN a1.id AS account_1, a2.id AS account_2, c.id AS shared_card
        ORDER BY account_1, account_2
        "#,
    )?;

    println!();
    println!("accounts sharing a payment card:");
    table.for_each_row(|row| {
        let account_1 = match row[0] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let account_2 = match row[1] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };
        let shared_card = match row[2] {
            CellRef::Text(bytes) => std::str::from_utf8(bytes).unwrap(),
            _ => "<unexpected>",
        };

        println!("  {account_1} <-> {account_2} via {shared_card}");
        Ok(())
    })?;

    Ok(())
}
