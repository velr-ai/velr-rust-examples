use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

use arrow2::array::{Array, Utf8Array};
use velr::{Result, Velr};

const TICKET_BIND: &str = "_tickets";

const IMPORT_TICKETS_QUERY: &str = r#"
UNWIND BIND('_tickets') AS r
CREATE (:Ticket {
    title: r.title,
    reporter: r.reporter,
    summary: r.summary,
    body: r.body,
    keywords: r.keywords
})
"#;

#[derive(Debug, Clone)]
struct TicketRow {
    title: String,
    reporter: String,
    summary: String,
    body: String,
    keywords: String, // stored as comma-separated text for simplicity
}

fn boxed_utf8(values: &[String]) -> Box<dyn Array> {
    let refs: Vec<Option<&str>> = values.iter().map(|s| Some(s.as_str())).collect();
    Utf8Array::<i64>::from(refs).boxed()
}

fn pick<'a, T>(rng: &mut StdRng, items: &'a [T]) -> &'a T {
    items.choose(rng).expect("non-empty slice")
}

fn jira_like_ticket(rng: &mut StdRng, id: u64) -> TicketRow {
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

    let first_names = [
        "Anna", "Johan", "Maria", "Erik", "Sofia", "Lina", "David", "Emma", "Oskar", "Sara",
    ];
    let last_names = [
        "Karlsson",
        "Nilsson",
        "Andersson",
        "Johansson",
        "Lindberg",
        "Svensson",
        "Larsson",
        "Berg",
        "Holm",
        "Dahl",
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

    let first = pick(rng, &first_names);
    let last = pick(rng, &last_names);
    let reporter = format!(
        "{}.{}@example.com",
        first.to_lowercase(),
        last.to_lowercase()
    );

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
Reporter: {} {}
Environment: {}
Component: {}
Priority: {}
Status: {}

Description:
A Jira-like synthetic ticket generated for load and import testing. The {} area {} {}. {}

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
        first,
        last,
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
        title,
        reporter,
        summary,
        body,
        keywords,
    }
}

fn build_ticket_batch(rng: &mut StdRng, start_id: u64, batch_size: usize) -> Vec<TicketRow> {
    let mut rows = Vec::with_capacity(batch_size);
    for i in 0..batch_size {
        rows.push(jira_like_ticket(rng, start_id + i as u64));
    }
    rows
}

fn import_ticket_batch(db: &Velr, rows: &[TicketRow]) -> Result<()> {
    let titles: Vec<String> = rows.iter().map(|r| r.title.clone()).collect();
    let reporters: Vec<String> = rows.iter().map(|r| r.reporter.clone()).collect();
    let summaries: Vec<String> = rows.iter().map(|r| r.summary.clone()).collect();
    let bodies: Vec<String> = rows.iter().map(|r| r.body.clone()).collect();
    let keywords: Vec<String> = rows.iter().map(|r| r.keywords.clone()).collect();

    let cols = vec![
        "title".to_string(),
        "reporter".to_string(),
        "summary".to_string(),
        "body".to_string(),
        "keywords".to_string(),
    ];

    let arrays: Vec<Box<dyn Array>> = vec![
        boxed_utf8(&titles),
        boxed_utf8(&reporters),
        boxed_utf8(&summaries),
        boxed_utf8(&bodies),
        boxed_utf8(&keywords),
    ];

    db.bind_arrow(TICKET_BIND, cols, arrays)?;
    db.run(IMPORT_TICKETS_QUERY)?;

    Ok(())
}
pub fn import_synthetic_tickets(
    db: &Velr,
    total_tickets: usize,
    batch_size: usize,
    seed: u64,
) -> Result<()> {
    use std::time::Instant;

    assert!(batch_size > 0, "batch_size must be > 0");

    let total_start = Instant::now();
    let mut rng = StdRng::seed_from_u64(seed);
    let mut imported = 0usize;
    let mut start_id = 1u64;
    let mut batch_no = 0usize;

    while imported < total_tickets {
        let batch_start = Instant::now();

        let this_batch = std::cmp::min(batch_size, total_tickets - imported);
        let rows = build_ticket_batch(&mut rng, start_id, this_batch);

        import_ticket_batch(db, &rows)?;

        imported += this_batch;
        start_id += this_batch as u64;
        batch_no += 1;

        let batch_elapsed = batch_start.elapsed();
        let total_elapsed = total_start.elapsed();

        let batch_rows_per_sec = this_batch as f64 / batch_elapsed.as_secs_f64();
        let total_rows_per_sec = imported as f64 / total_elapsed.as_secs_f64();

        println!(
            "Imported batch {}: {} rows in {:?} ({:.0} rows/sec), total: {}/{} in {:?} ({:.0} rows/sec)",
            batch_no,
            this_batch,
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

#[test]
fn import_ticket_batches_smoke() -> Result<()> {
    use std::time::Instant;

    let total_start = Instant::now();

    let open_start = Instant::now();
    let db = Velr::open(Some("velr-tickets.velr"))?;
    println!("open took: {:?}", open_start.elapsed());

    let import_start = Instant::now();
    import_synthetic_tickets(&db, 1_000_000, 100_000, 42)?;
    println!("import took: {:?}", import_start.elapsed());

    println!("total took: {:?}", total_start.elapsed());

    Ok(())
}

#[test]
fn small_example_preview() -> Result<()> {
    let db = Velr::open(None)?;

    import_synthetic_tickets(&db, 10, 5, 42)?;

    let mut t = db.exec_one(
        r#"
        MATCH (t:Ticket)
        RETURN t.title AS title, t.reporter AS reporter, t.keywords AS keywords
        ORDER BY title
        "#,
    )?;
    let ipc = t.to_arrow_ipc_file()?;
    println!("Preview IPC bytes: {}", ipc.len());

    Ok(())
}
