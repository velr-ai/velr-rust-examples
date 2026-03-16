use std::{fs, time::Instant};

use arrow2::array::{Array, Utf8Array};
use rand::{SeedableRng, rngs::StdRng, seq::SliceRandom};
use velr::{CellRef, Result, Velr};

const DB_PATH: &str = "jira_tickets.db";
const TICKET_BIND: &str = "_tickets";

const TOTAL_TICKETS: usize = 10_000;
const BATCH_SIZE: usize = 1_000;
const RNG_SEED: u64 = 42;

const IMPORT_TICKETS_QUERY: &str = r#"
UNWIND BIND('_tickets') AS r
CREATE (:Ticket {
    ticket_key: r.ticket_key,
    title: r.title,
    reporter: r.reporter,
    summary: r.summary,
    body: r.body,
    keywords: r.keywords
})
"#;

#[derive(Debug, Clone)]
struct TicketRow {
    ticket_key: String,
    title: String,
    reporter: String,
    summary: String,
    body: String,
    keywords: String, // stored as comma-separated text for simplicity
}

#[derive(Debug, Clone, Copy)]
struct Reporter {
    display_name: &'static str,
    email: &'static str,
}

fn boxed_utf8(values: &[String]) -> Box<dyn Array> {
    let refs: Vec<Option<&str>> = values.iter().map(|s| Some(s.as_str())).collect();
    Utf8Array::<i64>::from(refs).boxed()
}

fn pick<'a, T>(rng: &mut StdRng, items: &'a [T]) -> &'a T {
    items.choose(rng).expect("non-empty slice")
}

fn cell_as_str<'a>(cell: &'a CellRef<'a>) -> &'a str {
    match cell {
        CellRef::Text(bytes) | CellRef::Json(bytes) => std::str::from_utf8(bytes).unwrap(),
        _ => "<unexpected>",
    }
}

fn synthetic_ticket(rng: &mut StdRng, id: u64) -> TicketRow {
    let issue_types = ["BUG", "STORY", "TASK", "IMPROVEMENT", "SUPPORT"];
    let priorities = ["Low", "Medium", "High", "Critical"];
    let statuses = ["Open", "In Progress", "Blocked", "Ready for QA"];

    let components = [
        "Authentication",
        "Search",
        "Billing",
        "Reporting",
        "Notifications",
        "Mobile App",
        "API Gateway",
        "Data Pipeline",
        "Permissions",
        "Dashboard",
    ];

    let environments = [
        "production",
        "staging",
        "EU region",
        "US region",
        "mobile web",
        "desktop web",
        "internal admin",
        "partner portal",
    ];

    let verbs = [
        "fails",
        "times out",
        "returns incorrect data",
        "shows stale information",
        "duplicates results",
        "drops updates",
        "blocks submission",
        "renders incorrectly",
        "logs unexpected warnings",
        "creates inconsistent state",
    ];

    let objects = [
        "during login",
        "when saving a draft",
        "while searching by keyword",
        "after a deployment",
        "when uploading attachments",
        "during report generation",
        "while syncing records",
        "when opening the dashboard",
        "during pagination",
        "when retrying a request",
    ];

    let impacts = [
        "This impacts multiple users and creates confusion in day-to-day operations.",
        "The issue is intermittent but has been reported by several customers.",
        "This appears to affect the main user flow and should be prioritized.",
        "The defect is causing support tickets and manual workarounds.",
        "The behavior is visible in demos and affects trust in the product.",
    ];

    let expected_results = [
        "The operation should complete successfully and persist the latest user input.",
        "The UI should show the latest data without requiring a manual refresh.",
        "The request should return a consistent result set across repeated calls.",
        "The action should succeed without duplicate side effects.",
        "The page should render correctly and remain responsive.",
    ];

    let actual_results = [
        "Instead, the request fails with an unexpected error.",
        "Instead, the UI shows outdated information for several seconds.",
        "Instead, duplicate records are created in some cases.",
        "Instead, the action hangs until the client times out.",
        "Instead, users see partial data and need to retry manually.",
    ];

    let reproduce_steps = [
        "Open the affected area in the application.",
        "Perform the main user action with realistic input.",
        "Observe the system response and compare it with the expected result.",
        "Repeat the flow with the same data to confirm the behavior.",
    ];

    let reporters = [
        Reporter {
            display_name: "Frodo Baggins",
            email: "frodo.baggins@example.com",
        },
        Reporter {
            display_name: "Samwise Gamgee",
            email: "samwise.gamgee@example.com",
        },
        Reporter {
            display_name: "Aragorn",
            email: "aragorn@example.com",
        },
        Reporter {
            display_name: "Arwen",
            email: "arwen@example.com",
        },
        Reporter {
            display_name: "Legolas",
            email: "legolas@example.com",
        },
        Reporter {
            display_name: "Gimli",
            email: "gimli@example.com",
        },
        Reporter {
            display_name: "Gandalf",
            email: "gandalf@example.com",
        },
        Reporter {
            display_name: "Boromir",
            email: "boromir@example.com",
        },
        Reporter {
            display_name: "Éowyn",
            email: "eowyn@example.com",
        },
        Reporter {
            display_name: "Faramir",
            email: "faramir@example.com",
        },
    ];

    let tags = [
        "bug",
        "story",
        "task",
        "api",
        "ui",
        "backend",
        "frontend",
        "auth",
        "search",
        "billing",
        "reporting",
        "performance",
        "security",
        "regression",
        "sync",
        "timeout",
        "data-quality",
        "jira-import",
        "triage",
        "customer-reported",
    ];

    let issue_type = pick(rng, &issue_types);
    let priority = pick(rng, &priorities);
    let status = pick(rng, &statuses);
    let component = pick(rng, &components);
    let environment = pick(rng, &environments);
    let verb = pick(rng, &verbs);
    let object = pick(rng, &objects);
    let impact = pick(rng, &impacts);
    let expected = pick(rng, &expected_results);
    let actual = pick(rng, &actual_results);
    let reporter = pick(rng, &reporters);

    let title = format!("[{}] {} {} in {}", issue_type, component, verb, environment);
    let summary = format!(
        "{} {} {}. Priority: {}. Status: {}.",
        component, verb, object, priority, status
    );

    let mut keyword_list: Vec<&str> = tags.choose_multiple(rng, 4).cloned().collect();
    keyword_list.push(component);
    keyword_list.push(environment);
    let keywords = keyword_list.join(", ");

    let ticket_key = format!("SYN-{}", id);

    let body = format!(
        r#"Issue Key: {}
Reporter: {}
Reporter Email: {}
Environment: {}
Component: {}
Priority: {}
Status: {}

Description:
A Jira-like synthetic ticket generated for import testing. The {} area {} {}. {}

Steps to Reproduce:
1. {}
2. {}
3. {}
4. {}

Expected Result:
{}

Actual Result:
{}

Notes:
This is synthetic but written to resemble a real issue ticket with natural language text.
"#,
        ticket_key,
        reporter.display_name,
        reporter.email,
        environment,
        component,
        priority,
        status,
        component,
        verb,
        object,
        impact,
        reproduce_steps[0],
        reproduce_steps[1],
        reproduce_steps[2],
        reproduce_steps[3],
        expected,
        actual,
    );

    TicketRow {
        ticket_key,
        title,
        reporter: reporter.email.to_string(),
        summary,
        body,
        keywords,
    }
}

fn build_ticket_batch(rng: &mut StdRng, start_id: u64, batch_size: usize) -> Vec<TicketRow> {
    let mut rows = Vec::with_capacity(batch_size);
    for offset in 0..batch_size {
        rows.push(synthetic_ticket(rng, start_id + offset as u64));
    }
    rows
}

fn import_ticket_batch(db: &Velr, rows: &[TicketRow]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let ticket_keys: Vec<String> = rows.iter().map(|r| r.ticket_key.clone()).collect();
    let titles: Vec<String> = rows.iter().map(|r| r.title.clone()).collect();
    let reporters: Vec<String> = rows.iter().map(|r| r.reporter.clone()).collect();
    let summaries: Vec<String> = rows.iter().map(|r| r.summary.clone()).collect();
    let bodies: Vec<String> = rows.iter().map(|r| r.body.clone()).collect();
    let keywords: Vec<String> = rows.iter().map(|r| r.keywords.clone()).collect();

    let columns = vec![
        "ticket_key".to_string(),
        "title".to_string(),
        "reporter".to_string(),
        "summary".to_string(),
        "body".to_string(),
        "keywords".to_string(),
    ];

    let arrays: Vec<Box<dyn Array>> = vec![
        boxed_utf8(&ticket_keys),
        boxed_utf8(&titles),
        boxed_utf8(&reporters),
        boxed_utf8(&summaries),
        boxed_utf8(&bodies),
        boxed_utf8(&keywords),
    ];

    // Bind one batch as an Arrow-backed logical table, then import it with Cypher.
    db.bind_arrow(TICKET_BIND, columns, arrays)?;
    db.run(IMPORT_TICKETS_QUERY)?;

    Ok(())
}

fn import_synthetic_tickets(
    db: &Velr,
    total_tickets: usize,
    batch_size: usize,
    seed: u64,
) -> Result<()> {
    assert!(batch_size > 0, "batch_size must be > 0");

    let total_start = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut imported = 0usize;
    let mut start_id = 1u64;
    let mut batch_no = 0usize;

    while imported < total_tickets {
        let batch_start = Instant::now();

        let rows_in_batch = std::cmp::min(batch_size, total_tickets - imported);
        let rows = build_ticket_batch(&mut rng, start_id, rows_in_batch);

        import_ticket_batch(db, &rows)?;

        imported += rows_in_batch;
        start_id += rows_in_batch as u64;
        batch_no += 1;

        let batch_elapsed = batch_start.elapsed();
        let total_elapsed = total_start.elapsed();

        let batch_rows_per_sec = rows_in_batch as f64 / batch_elapsed.as_secs_f64();
        let total_rows_per_sec = imported as f64 / total_elapsed.as_secs_f64();

        println!(
            "batch {}: {} rows in {:?} ({:.0} rows/sec), total: {}/{} in {:?} ({:.0} rows/sec)",
            batch_no,
            rows_in_batch,
            batch_elapsed,
            batch_rows_per_sec,
            imported,
            total_tickets,
            total_elapsed,
            total_rows_per_sec,
        );
    }

    Ok(())
}

fn preview_import(db: &Velr) -> Result<()> {
    let mut count_table = db.exec_one(
        r#"
        MATCH (t:Ticket)
        RETURN count(t) AS tickets
        "#,
    )?;

    count_table.for_each_row(|row| {
        if let CellRef::Integer(n) = row[0] {
            println!("imported tickets: {n}");
        }
        Ok(())
    })?;

    let mut preview = db.exec_one(
        r#"
        MATCH (t:Ticket)
        RETURN t.ticket_key AS ticket_key, t.title AS title, t.reporter AS reporter
        ORDER BY ticket_key
        LIMIT 5
        "#,
    )?;

    println!();
    println!("preview:");
    preview.for_each_row(|row| {
        let ticket_key = cell_as_str(&row[0]);
        let title = cell_as_str(&row[1]);
        let reporter = cell_as_str(&row[2]);

        println!("  {ticket_key}: {title} ({reporter})");
        Ok(())
    })?;

    Ok(())
}

// cargo run --example batch_import --features arrow-ipc
fn main() -> Result<()> {
    // Remove any previous database file so rerunning the example starts cleanly.
    let _ = fs::remove_file(DB_PATH);

    println!("opening database: {DB_PATH}");
    let db = Velr::open(Some(DB_PATH))?;

    println!(
        "importing {} synthetic Jira-like tickets in batches of {}",
        TOTAL_TICKETS, BATCH_SIZE
    );

    let import_start = Instant::now();
    import_synthetic_tickets(&db, TOTAL_TICKETS, BATCH_SIZE, RNG_SEED)?;
    println!("import finished in {:?}", import_start.elapsed());

    println!();
    println!("database written to: {DB_PATH}");
    preview_import(&db)?;

    Ok(())
}
