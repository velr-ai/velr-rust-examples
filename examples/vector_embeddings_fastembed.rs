use std::{
    cell::RefCell,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use velr::{CellRef, PropertyValue, Velr};

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

fn input_text(input: &velr::VectorEmbeddingInput) -> String {
    input
        .fields
        .iter()
        .filter_map(|field| match &field.value {
            PropertyValue::String(value) => Some(value.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Vector indexes use sidecar storage, so they need a file-backed DB.
    let db_path = temp_db_path("vector_embeddings");
    let db = Velr::open(Some(&db_path))?;

    // BAAI/bge-small-en-v1.5 is a small 384-dimensional text embedding model.
    // The first run downloads the model into FastEmbed's normal cache.
    let model = Rc::new(RefCell::new(TextEmbedding::try_new(
        InitOptions::new(EmbeddingModel::BGESmallENV15).with_show_download_progress(true),
    )?));

    let model_for_callback = Rc::clone(&model);
    db.register_vector_embedder("fastembed", move |inputs| {
        let documents = inputs.iter().map(input_text).collect::<Vec<_>>();
        model_for_callback
            .borrow_mut()
            .embed(documents, None)
            .map_err(|err| err.to_string())
    })?;

    db.run(
        r#"
        CREATE VECTOR INDEX paperEmbedding IF NOT EXISTS
        FOR (n:Paper)
        ON EACH [n.title, n.abstract]
        OPTIONS {
          indexConfig: {
            dimensions: 384,
            metric: 'cosine',
            embedder: 'fastembed'
          }
        }
        "#,
    )?;

    // The index is maintained by normal writes. There is no separate
    // "add this vector" bookkeeping step.
    db.run(
        r#"
        CREATE
          (:Paper {
            id: 'p1',
            title: 'Graph Search with Rust',
            abstract: 'Index nodes and relationships with openCypher'
          }),
          (:Paper {
            id: 'p2',
            title: 'Greek Letters in Graph Search',
            abstract: 'Alpha, beta, and gamma examples for retrieval'
          }),
          (:Paper {
            id: 'p3',
            title: 'Local Embeddings',
            abstract: 'Run compact embedding models inside an application'
          })
        "#,
    )?;

    db.run(
        r#"
        MATCH (p:Paper {id: 'p2'})
        SET p.abstract = p.abstract + ' with semantic vector search'
        "#,
    )?;

    let mut table = db.exec_one(
        r#"
        CALL db.index.vector.queryNodes('paperEmbedding', 3, 'paper about greek letters')
        YIELD node, score
        RETURN node, score
        "#,
    )?;

    println!("nearest papers:");
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
