use std::time::{SystemTime, UNIX_EPOCH};

use velr::{CellRef, Velr};

fn temp_db_path(name: &str) -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after UNIX_EPOCH")
        .as_nanos();
    std::env::temp_dir()
        .join(format!("velr_{name}_{}_{}.db", std::process::id(), nanos))
        .to_string_lossy()
        .into_owned()
}

fn display_cell(cell: CellRef<'_>) -> String {
    match cell {
        CellRef::Null => "null".to_string(),
        CellRef::Bool(value) => value.to_string(),
        CellRef::Integer(value) => value.to_string(),
        CellRef::Float(value) => value.to_string(),
        CellRef::Text(bytes) | CellRef::Json(bytes) => String::from_utf8_lossy(bytes).into_owned(),
    }
}

fn main() -> velr::Result<()> {
    // Full-text indexes use sidecar storage, so they need a file-backed DB.
    let db_path = temp_db_path("fulltext_search");
    let db = Velr::open(Some(&db_path))?;

    db.run(
        r#"
        CREATE
          (:Paper {
            id: 'p1',
            title: 'Graph Search with Rust',
            abstract: 'A practical tour of graph retrieval and indexing'
          }),
          (:Paper {
            id: 'p2',
            title: 'Vector Embeddings for Local Apps',
            abstract: 'Semantic retrieval with compact embedding models'
          }),
          (:Paper {
            id: 'p3',
            title: 'Greek Letters in Scientific Papers',
            abstract: 'Alpha, beta, and gamma notation in research writing'
          })
        "#,
    )?;

    db.run(
        r#"
        CREATE FULLTEXT INDEX paperText IF NOT EXISTS
        FOR (n:Paper)
        ON EACH [n.title, n.abstract]
        "#,
    )?;

    println!("full-text search for title:graph OR abstract:embedding");
    let mut table = db.exec_one(
        r#"
        CALL db.index.fulltext.queryNodes('paperText', 'title:graph OR abstract:embedding')
        YIELD node, score
        RETURN node, score
        "#,
    )?;

    table.for_each_row(|row| {
        let node = display_cell(row[0]);
        let score = match row[1] {
            CellRef::Float(value) => value,
            _ => f64::NAN,
        };
        println!("  node={node} score={score:.3}");
        Ok(())
    })?;

    // Writes keep the full-text index current. No manual reindex call is needed.
    db.run(
        r#"
        MATCH (p:Paper {id: 'p3'})
        SET p.abstract = 'Greek letters used in graph embeddings and search examples'
        "#,
    )?;

    println!();
    println!("after updating p3.abstract:");
    let mut table = db.exec_one(
        r#"
        CALL db.index.fulltext.queryNodes('paperText', 'abstract:embeddings')
        YIELD node, score
        RETURN node, score
        "#,
    )?;

    table.for_each_row(|row| {
        let node = display_cell(row[0]);
        let score = match row[1] {
            CellRef::Float(value) => value,
            _ => f64::NAN,
        };
        println!("  node={node} score={score:.3}");
        Ok(())
    })?;

    Ok(())
}
