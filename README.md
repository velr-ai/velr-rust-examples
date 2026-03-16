# velr-rust-examples

Examples showing how to use the [Velr](https://velr.ai/) Rust driver.

Velr is an embedded property-graph database built in Rust, backed by SQLite, and queried with openCypher. This repository contains small, focused Rust examples that demonstrate common patterns when working with the Velr Rust API.

## What this repo contains

Examples in this repository cover:

- opening in-memory and file-backed databases
- creating and querying graph data
- reading result tables row by row
- handling typed cell values
- streaming multiple result tables
- using transactions, rollbacks, and savepoints
- inspecting query plans with `explain()`
- working with openCypher concepts such as `MATCH`, `WHERE`, `MERGE`, `WITH`, paths, and variable-length paths
- modeling real-world graph use cases such as knowledge graphs, fraud detection, access control, org charts, and ticket dependencies
- importing data with Arrow and `UNWIND BIND(...)`

## Getting started

Clone the repository and run an example with Cargo:

```bash
git clone https://github.com/velr-ai/velr-rust-examples.git
cd velr-rust-examples
cargo run --example basic_query
````

Some examples require the `arrow-ipc` feature:

```bash
cargo run --example batch_import --features arrow-ipc
```

## Example layout

Examples live in the `examples/` directory.

### Core driver examples

* `basic_open.rs`
* `basic_query.rs`
* `file_backed.rs`
* `streaming_tables.rs`
* `transaction.rs`
* `rollback.rs`
* `rollback_on_drop.rs`
* `savepoints.rs`
* `explain.rs`

### openCypher examples

* `cypher_match.rs`
* `cypher_where.rs`
* `cypher_relationships.rs`
* `cypher_aggregates.rs`
* `cypher_unwind.rs`
* `cypher_labels_and_properties.rs`
* `cypher_paths.rs`
* `cypher_var_length_paths.rs`
* `cypher_merge.rs`
* `cypher_merge_relationships.rs`
* `cypher_with.rs`
* `cypher_with_aggregates.rs`

### Use case examples

* `usecase_knowledge_graph.rs`
* `usecase_fraud_detection.rs`
* `usecase_ticket_dependencies.rs`
* `usecase_access_control.rs`
* `usecase_org_chart.rs`

### Arrow examples

* `arrow_ipc.rs`
* `arrow_chunked.rs`
* `batch_import.rs`

## Minimal example

```rust
use velr::{CellRef, Velr};

fn main() -> velr::Result<()> {
    let db = Velr::open(None)?;

    db.run("CREATE (:Person {name:'Keanu Reeves', born:1964})")?;

    let mut table = db.exec_one("MATCH (p:Person) RETURN p.name AS name, p.born AS born")?;

    table.for_each_row(|row| {
        if let CellRef::Text(bytes) = row[0] {
            println!("name={}", std::str::from_utf8(bytes).unwrap());
        }

        if let CellRef::Integer(i) = row[1] {
            println!("born={i}");
        }

        Ok(())
    })?;

    Ok(())
}
```

## Batch import example

The `batch_import.rs` example shows how to import synthetic Jira-like tickets into Velr in batches.

It demonstrates how to:

* generate a synthetic dataset in Rust
* bind batch data with `bind_arrow`
* import rows with `UNWIND BIND(...)`
* write the result to a file-backed Velr database
* run a small preview query after the import

Run it with:

```bash
cargo run --example batch_import --features arrow-ipc
```

By default, the example writes a local database file named `jira_tickets.velr`, imports synthetic `Ticket` nodes in batches, and then prints a small preview of the imported data.

## License

This repository is licensed under the MIT License. See [`LICENSE`](LICENSE).