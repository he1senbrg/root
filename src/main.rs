use axum::{routing::get, Router};
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    // 12/1/25: Going to assume this is only necessary for shuttle
    // 9/1/25: TODO: Explain?
    // env::set_var("PGOPTIONS", "-c ignore_version=true");

    // let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
    //  .data(pool.clone())
    //  .data(secret_key.clone())
    //  .finish();

    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Pool must be initialized properly.");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations.");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    info!("Starting Root...");
    let router = Router::new().route("/", get(root)).layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();

    // task::spawn(async move {
    //  schedule_task_at_midnight(pool.clone()).await;
    // });
}

async fn root() -> &'static str {
    "Hello, world!"
}

// Sleep till midnight, then execute the task, repeat.
// async fn schedule_task_at_midnight(pool: Arc<PgPool>) {
//     loop {
//         let now = Local::now();
//         let next_midnight = (now + chrono::Duration::days(1))
//             .date_naive()
//             .and_hms_opt(0, 0, 0)
//             .unwrap();
//
//         let duration_until_midnight = next_midnight.signed_duration_since(now.naive_local());
//         let sleep_duration =
//             tokio::time::Duration::from_secs(duration_until_midnight.num_seconds() as u64);
//
//         sleep_until(Instant::now() + sleep_duration).await;
//         scheduled_task(pool.clone()).await;
//    }
// }
