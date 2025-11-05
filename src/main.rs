mod payloads;

use std::error::Error;

use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use payloads::{CreateInstancePayload, CreateQuizPayload, PostAnswerPayload, UpdateInstanceStatePayload};
use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pg_pool = PgPool::connect(&std::env::var("DATABASE_URL")?.clone()).await?;
    sqlx::migrate!("./migrations").run(&pg_pool).await?;
    let router = Router::new()
        .route("/quiz", routing::post(create_quiz).get(get_all_quizzes))
        .route("/quiz/:quiz_id", routing::get(get_quiz).post(update_quiz).delete(delete_quiz))
        .route("/quiz/:quiz_id/instance", routing::post(create_instance))
        .route("/quiz/instance/:instance_id", routing::get(get_instance).delete(delete_instance))
        .route("/quiz/instance/:instance_id/state", routing::post(update_instance_state))
        .route("/quiz/instance/:instance_id/answer", routing::post(post_answer));
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn create_quiz(Json(payload): Json<CreateQuizPayload>) -> impl IntoResponse {
    // TODO
    println!("{payload:?}");
    (StatusCode::CREATED, Json(()))
}

async fn get_all_quizzes() -> impl IntoResponse {
    // TODO
    (StatusCode::OK, Json(()))
}

async fn get_quiz(Path(quiz_id): Path<String>) -> impl IntoResponse {
    // TODO
    println!("{quiz_id:?}");
    (StatusCode::OK, Json(()))
}

async fn update_quiz(Path(quiz_id): Path<String>, Json(payload): Json<CreateQuizPayload>) -> impl IntoResponse {
    // TODO
    println!("{quiz_id:?}");
    println!("{payload:?}");
    (StatusCode::OK, Json(()))
}

async fn delete_quiz(Path(quiz_id): Path<String>) -> impl IntoResponse {
    // TODO
    println!("{quiz_id:?}");
    (StatusCode::OK, Json(()))
}

async fn create_instance(Path(quiz_id): Path<String>, Json(payload): Json<CreateInstancePayload>) -> impl IntoResponse {
    // TODO
    println!("{quiz_id:?}");
    println!("{payload:?}");
    (StatusCode::CREATED, Json(()))
}

async fn get_instance(Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    println!("{instance_id:?}");
    (StatusCode::OK, Json(()))
}

async fn delete_instance(Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    println!("{instance_id:?}");
    (StatusCode::OK, Json(()))
}

async fn update_instance_state(
    Path(instance_id): Path<String>,
    Json(payload): Json<UpdateInstanceStatePayload>,
) -> impl IntoResponse {
    // TODO
    println!("{instance_id:?}");
    println!("{payload:?}");
    (StatusCode::OK, Json(()))
}

async fn post_answer(Path(instance_id): Path<String>, Json(payload): Json<PostAnswerPayload>) -> impl IntoResponse {
    // TODO
    println!("{instance_id:?}");
    println!("{payload:?}");
    (StatusCode::OK, Json(()))
}
