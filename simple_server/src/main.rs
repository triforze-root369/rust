use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, delete},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info;

// Define the Task struct that represents our task data model
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    id: u32,
    title: String,
    completed: bool,
}

// Define the payload for creating a new task
#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

// Define our application state that will be shared between handlers
type Tasks = Arc<RwLock<Vec<Task>>>;

#[tokio::main]
async fn main() {
    // Initialize tracing for request logging
    tracing_subscriber::fmt::init();

    // Create shared state for tasks
    let tasks = Arc::new(RwLock::new(Vec::new()));

    // Create CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any);

    // Build our application with routes
    let app = Router::new()
        // API routes
        .route("/api/tasks", get(get_tasks))
        .route("/api/tasks", post(create_task))
        .route("/api/tasks/:id", delete(delete_task))
        // Serve static files
        .nest_service("/", ServeDir::new("static"))
        // Add middleware
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(tasks);

    // Start the server
    let addr = "127.0.0.1:3000";
    info!("Server starting on http://{}", addr);
    
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Handler for getting all tasks
async fn get_tasks(State(tasks): State<Tasks>) -> impl IntoResponse {
    let tasks = tasks.read().unwrap();
    Json(tasks.clone())
}

// Handler for creating a new task
async fn create_task(
    State(tasks): State<Tasks>,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, StatusCode> {
    // Validate task title length
    if payload.title.len() > 100 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tasks = tasks.write().unwrap();

    // Check if we've reached the maximum number of tasks
    if tasks.len() >= 1000 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Create new task
    let task = Task {
        id: tasks.len() as u32 + 1,
        title: payload.title,
        completed: false,
    };

    // Add task to storage
    tasks.push(task.clone());

    Ok((StatusCode::CREATED, Json(task)))
}

// Handler for deleting a task
async fn delete_task(
    State(tasks): State<Tasks>,
    Path(id): Path<u32>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut tasks = tasks.write().unwrap();
    
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
