use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod models;
mod handlers;

// Application state
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

// Example response struct
#[derive(Serialize, Deserialize)]
pub struct HealthResponse {
    status: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "lepsi_nez_a_backend=debug,tower_http=debug,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenvy::dotenv().ok();

    // Database connection
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/lepsi_nez_a".to_string());
    
    tracing::info!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await?;

    tracing::info!("Database connected successfully");

    // Create application state
    let state = AppState { db: pool };

    // Build router
    let app = Router::new()
        // Basic routes
        .route("/", get(root))
        .route("/health", get(health_check))
        
        // Quiz routes
        .route("/quiz", post(handlers::create_quiz))
        .route("/quiz", get(handlers::get_all_quizzes))
        .route("/quiz/:id", get(handlers::get_quiz))
        .route("/quiz/:id", post(handlers::update_quiz))
        .route("/quiz/:id", delete(handlers::delete_quiz))
        .route("/quiz/:id/instance", post(handlers::create_instance))
        
        // Quiz Instance routes
        .route("/quiz/instance/:instanceId", get(handlers::get_instance))
        .route("/quiz/instance/:instanceId", delete(handlers::delete_instance))
        .route("/quiz/instance/:instanceId/state", post(handlers::update_instance_state))
        .route("/quiz/instance/:instanceId/answer", post(handlers::post_answer))
        
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = "0.0.0.0:6767";
    tracing::info!("Starting server on {}", addr);
    let listener = TcpListener::bind(addr).await?;
    
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

// Root endpoint
async fn root() -> &'static str {
    "Hello, World!"
}

// Health check endpoint
async fn health_check(State(state): State<AppState>) -> impl IntoResponse {
    // Try to execute a simple query to check database connectivity
    match sqlx::query("SELECT 1")
        .fetch_one(&state.db)
        .await
    {
        Ok(_) => (
            StatusCode::OK,
            Json(HealthResponse {
                status: "healthy".to_string(),
            }),
        ),
        Err(e) => {
            tracing::error!("Health check failed: {}", e);
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(HealthResponse {
                    status: format!("unhealthy: {}", e),
                }),
            )
        }
    }
}

// Graceful shutdown handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Signal received, starting graceful shutdown");
}
