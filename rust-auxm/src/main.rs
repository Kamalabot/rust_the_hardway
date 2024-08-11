use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, patch},
    Json, Router,
};

use serde::{Deserializer, Serializer};
use serde_json::json;

use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

// use dotenvy;

#[tokio::main]
async fn main() {
    // expose the env vars
    dotenvy::dotenv().expect("Unable to access .env file");    

    // set the variables from the env files
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());
    let database_url = std::env::var("DATABASE_URL").expect("Unable to locate the url");

    // initiate the db_pool 
    let db_pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(&database_url)
        .await
        .expect("Can't connect to database");
    
    let listener = TcpListener::bind(server_address)
        .await
        .expect("Could not create TCP Listener");

    println!("Listening on {}", listener.local_addr().unwrap());

    let app = Router::new()
        .route("/", get(|| async {"hello world"}))
        .route("/tasks", get(get_tasks).post(create_task))
        .with_state(db_pool);

    axum::serve(listener, app)
        .await
        .expect("Error Server Application");
}

struct TaskRow {
    task_id: i32,
    name: String,
    priority: Option<i32>
}

async fn get_tasks(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    // need to understand the process under going here
    sqlx::query_as(TaskRow, "SELECT * FROM tasks ORDER BY task_id")
        .fetch_all(&pg_pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "success": false, "message": e.to_string()}).to_string(),
            )
        })?;
        
        Ok((
        StatusCode::OK,
        json!({ "success": true, "data": rows }).to_string(),
    ))
}

async fn create_task(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn update_task(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}

async fn delete_task(
    State(pg_pool): State<PgPool>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!()
}
