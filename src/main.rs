use std::error::Error;

use axum::{
    Router,
    extract::{Json, Path},
    http::StatusCode,
    response::IntoResponse,
    routing,
};
use serde::Deserialize;
// use sqlx::postgres::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let router = Router::new()
        .route("/quiz", routing::post(create_quiz).get(get_all_quizzes))
        .route("/quiz/:quiz_id", routing::get(get_quiz).post(update_quiz).delete(delete_quiz))
        .route("/quiz/:quiz_id/create_instance", routing::post(create_instance))
        .route("/instance/:instance_id", routing::get(get_instance).delete(delete_instance))
        .route("/instance/:instance_id/update_state", routing::post(update_instance_state))
        .route("/instance/:instance_id/post_answer/:team_id", routing::post(post_answer));
    let listener = TcpListener::bind("127.0.0.1:6767").await?;
    axum::serve(listener, router).await?;
    // let pg_pool = PgPool::connect(todo!()).await?;
    Ok(())
}

#[derive(Deserialize)]
struct CreateQuizPayload {} // TODO

#[derive(Deserialize)]
struct CreateInstancePayload {} // TODO

#[derive(Deserialize)]
struct UpdateInstanceStatePayload {} // TODO

#[derive(Deserialize)]
struct PostAnswerPayload {} // TODO

async fn create_quiz(Json(payload): Json<CreateQuizPayload>) -> impl IntoResponse {
    // TODO
    let _ = payload;
    (StatusCode::CREATED, Json(()))
}

async fn get_all_quizzes() -> impl IntoResponse {
    // TODO
    (StatusCode::OK, Json(()))
}

async fn get_quiz(Path(quiz_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    (StatusCode::OK, Json(()))
}

async fn update_quiz(Path(quiz_id): Path<String>, Json(payload): Json<CreateQuizPayload>) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}

async fn delete_quiz(Path(quiz_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    (StatusCode::OK, Json(()))
}

async fn create_instance(Path(quiz_id): Path<String>, Json(payload): Json<CreateInstancePayload>) -> impl IntoResponse {
    // TODO
    drop(quiz_id);
    let _ = payload;
    (StatusCode::CREATED, Json(()))
}

async fn get_instance(Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    (StatusCode::OK, Json(()))
}

async fn delete_instance(Path(instance_id): Path<String>) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    (StatusCode::OK, Json(()))
}

async fn update_instance_state(
    Path(instance_id): Path<String>,
    Json(payload): Json<UpdateInstanceStatePayload>,
) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}

async fn post_answer(
    Path((instance_id, team_id)): Path<(String, String)>,
    Json(payload): Json<PostAnswerPayload>,
) -> impl IntoResponse {
    // TODO
    drop(instance_id);
    drop(team_id);
    let _ = payload;
    (StatusCode::OK, Json(()))
}
