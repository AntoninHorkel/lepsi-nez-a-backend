use std::{env, error::Error};

use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use serde::Deserialize;
use sqlx::{Pool, Postgres, postgres::PgPool};
use tokio::net::TcpListener;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pg_pool = PgPool::connect(&env::var("DATABASE_URL")?.clone()).await?;
    sqlx::migrate!("./migrations").run(&pg_pool).await?;

    let router = Router::new()
        .route("/quiz", routing::post(create_quiz).get(get_all_quizzes))
        .route("/quiz/:quiz_id", routing::get(get_quiz).post(update_quiz).delete(delete_quiz))
        .route("/quiz/:quiz_id/instance", routing::post(create_instance))
        .route("/quiz/instance/:instance_id", routing::get(get_instance).delete(delete_instance))
        .route("/quiz/instance/:instance_id/state", routing::post(update_instance_state))
        .route("/quiz/instance/:instance_id/answer", routing::post(post_answer))
        .with_state(pg_pool);
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CreateQuizPayload {}

#[derive(Deserialize)]
struct CreateQuestionPayload {}

#[derive(Deserialize)]
struct CreateInstancePayload {}

#[derive(Deserialize)]
struct UpdateInstanceStatePayload {}

#[derive(Deserialize)]
struct PostAnswerPayload {}

async fn create_quiz(State(pool): State<Pool<Postgres>>, Json(payload): Json<CreateQuizPayload>) -> impl IntoResponse {
    // TODO
    let _ = payload;
    (StatusCode::CREATED, Json(()))
}

async fn get_all_quizzes(State(pool): State<Pool<Postgres>>) -> impl IntoResponse {
    // TODO
    (StatusCode::OK, Json(()))
}

async fn get_quiz(State(pool): State<Pool<Postgres>>, Path(quiz_id): Path<Uuid>) -> impl IntoResponse {
    // TODO

    (StatusCode::OK, Json(()))
}

async fn update_quiz(
    State(pool): State<Pool<Postgres>>,
    Path(quiz_id): Path<String>,
    Json(payload): Json<CreateQuizPayload>,
) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}

async fn delete_quiz(State(pool): State<Pool<Postgres>>, Path(quiz_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    (StatusCode::OK, Json(()))
}

async fn create_instance(
    State(pool): State<Pool<Postgres>>,
    Path(quiz_id): Path<String>,
    Json(payload): Json<CreateInstancePayload>,
) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    let _ = payload;
    (StatusCode::CREATED, Json(()))
}

async fn get_instance(State(pool): State<Pool<Postgres>>, Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    (StatusCode::OK, Json(()))
}

async fn delete_instance(State(pool): State<Pool<Postgres>>, Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    (StatusCode::OK, Json(()))
}

async fn update_instance_state(
    State(pool): State<Pool<Postgres>>,
    Path(instance_id): Path<String>,
    Json(payload): Json<UpdateInstanceStatePayload>,
) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}

async fn post_answer(
    State(pool): State<Pool<Postgres>>,
    Path(instance_id): Path<String>,
    Json(payload): Json<PostAnswerPayload>,
) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}
