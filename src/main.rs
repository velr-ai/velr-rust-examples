fn main() {
    println!("velr-rust-examples");
    println!();
    println!("Run an example with:");
    println!("  cargo run --example basic_query");
    println!();

    println!("Core driver examples:");
    println!("  cargo run --example basic_open");
    println!("  cargo run --example basic_query");
    println!("  cargo run --example file_backed");
    println!("  cargo run --example streaming_tables");
    println!("  cargo run --example transaction");
    println!("  cargo run --example rollback");
    println!("  cargo run --example rollback_on_drop");
    println!("  cargo run --example savepoints");
    println!("  cargo run --example explain");
    println!();

    println!("openCypher examples:");
    println!("  cargo run --example cypher_match");
    println!("  cargo run --example cypher_where");
    println!("  cargo run --example cypher_relationships");
    println!("  cargo run --example cypher_aggregates");
    println!("  cargo run --example cypher_unwind");
    println!("  cargo run --example cypher_labels_and_properties");
    println!("  cargo run --example cypher_paths");
    println!("  cargo run --example cypher_var_length_paths");
    println!("  cargo run --example cypher_merge");
    println!("  cargo run --example cypher_merge_relationships");
    println!("  cargo run --example cypher_with");
    println!("  cargo run --example cypher_with_aggregates");
    println!();

    println!("Use case examples:");
    println!("  cargo run --example usecase_knowledge_graph");
    println!("  cargo run --example usecase_fraud_detection");
    println!("  cargo run --example usecase_ticket_dependencies");
    println!("  cargo run --example usecase_access_control");
    println!("  cargo run --example usecase_org_chart");
    println!();

    println!("Arrow examples:");
    println!("  cargo run --example arrow_ipc --features arrow-ipc");
    println!("  cargo run --example arrow_chunked --features arrow-ipc");
    println!("  cargo run --example batch_import --features arrow-ipc");
}
