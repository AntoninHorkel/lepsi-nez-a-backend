use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use crate::models::*;
use crate::AppState;

// ==================== QUIZ ENDPOINTS ====================

// POST /quiz - Create a new quiz
pub async fn create_quiz(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuiz>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Quiz>(
        "INSERT INTO quiz (name) VALUES ($1) RETURNING id, name, created_at, updated_at"
    )
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    {
        Ok(quiz) => (StatusCode::CREATED, Json(quiz)),
        Err(e) => {
            tracing::error!("Failed to create quiz: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Quiz {
                    id: 0,
                    name: String::new(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                }),
            )
        }
    }
}

// GET /quiz - Get all quizzes
pub async fn get_all_quizzes(State(state): State<AppState>) -> impl IntoResponse {
    match sqlx::query_as::<_, Quiz>("SELECT * FROM quiz ORDER BY created_at DESC")
        .fetch_all(&state.db)
        .await
    {
        Ok(quizzes) => (StatusCode::OK, Json(quizzes)),
        Err(e) => {
            tracing::error!("Failed to fetch quizzes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

// GET /quiz/{id} - Get a specific quiz
pub async fn get_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<QuizWithQuestions>, StatusCode> {
    // Get quiz
    let quiz = sqlx::query_as::<_, Quiz>("SELECT * FROM quiz WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch quiz: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Get questions for quiz
    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM question WHERE quiz_id = $1 ORDER BY id"
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch questions: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(QuizWithQuestions { quiz, questions }))
}

// POST /quiz/{id} - Update a quiz
pub async fn update_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateQuiz>,
) -> Result<Json<Quiz>, StatusCode> {
    sqlx::query_as::<_, Quiz>(
        "UPDATE quiz SET name = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 
         RETURNING id, name, created_at, updated_at"
    )
    .bind(&payload.name)
    .bind(id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update quiz: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)
    .map(Json)
}

// DELETE /quiz/{id} - Delete a quiz
pub async fn delete_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM quiz WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete quiz: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// ==================== QUIZ INSTANCE ENDPOINTS ====================

// POST /quiz/{id}/instance - Create a quiz instance
pub async fn create_instance(
    State(state): State<AppState>,
    Path(quiz_id): Path<i32>,
) -> Result<Json<QuizInstance>, StatusCode> {
    // Verify quiz exists
    let quiz_exists = sqlx::query("SELECT id FROM quiz WHERE id = $1")
        .bind(quiz_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check quiz existence: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !quiz_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    sqlx::query_as::<_, QuizInstance>(
        "INSERT INTO quiz_instance (quiz_id) 
         VALUES ($1) 
         RETURNING id, uuid, quiz_id, created_at"
    )
    .bind(quiz_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create quiz instance: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })
    .map(Json)
}

// GET /quiz/instance/{instanceId} - Get a quiz instance by UUID
pub async fn get_instance(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
) -> Result<Json<QuizInstanceDetail>, StatusCode> {
    let uuid = uuid::Uuid::parse_str(&instance_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get quiz instance
    let instance = sqlx::query_as::<_, QuizInstance>(
        "SELECT * FROM quiz_instance WHERE uuid = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch quiz instance: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Get quiz details
    let quiz = sqlx::query_as::<_, Quiz>("SELECT * FROM quiz WHERE id = $1")
        .bind(instance.quiz_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch quiz: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Get questions
    let questions = sqlx::query_as::<_, Question>(
        "SELECT * FROM question WHERE quiz_id = $1 ORDER BY id"
    )
    .bind(instance.quiz_id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch questions: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get team answers
    let team_answers = sqlx::query_as::<_, TeamAnswer>(
        "SELECT * FROM team_answer WHERE quiz_instance_id = $1 ORDER BY num, question_id"
    )
    .bind(instance.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch team answers: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(QuizInstanceDetail {
        instance,
        quiz,
        questions,
        team_answers,
    }))
}

// DELETE /quiz/instance/{instanceId} - Delete a quiz instance
pub async fn delete_instance(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let uuid = uuid::Uuid::parse_str(&instance_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let result = sqlx::query("DELETE FROM quiz_instance WHERE uuid = $1")
        .bind(uuid)
        .execute(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete quiz instance: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

// POST /quiz/instance/{instanceId}/answer - Submit team answer
pub async fn post_answer(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
    Json(payload): Json<SubmitAnswer>,
) -> Result<Json<TeamAnswer>, StatusCode> {
    let uuid = uuid::Uuid::parse_str(&instance_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Get quiz instance
    let instance = sqlx::query_as::<_, QuizInstance>(
        "SELECT * FROM quiz_instance WHERE uuid = $1"
    )
    .bind(uuid)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch quiz instance: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    // Verify question exists and belongs to this quiz
    let question_exists = sqlx::query(
        "SELECT id FROM question WHERE id = $1 AND quiz_id = $2"
    )
    .bind(payload.question_id)
    .bind(instance.quiz_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to check question: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?
    .is_some();

    if !question_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // Insert or update team answer
    let team_answer = sqlx::query_as::<_, TeamAnswer>(
        "INSERT INTO team_answer (num, ans, question_id, quiz_instance_id)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (quiz_instance_id, question_id, num)
         DO UPDATE SET ans = EXCLUDED.ans, created_at = CURRENT_TIMESTAMP
         RETURNING id, num, ans, question_id, quiz_instance_id, created_at"
    )
    .bind(payload.team_id)
    .bind(&payload.ans)
    .bind(payload.question_id)
    .bind(instance.id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Failed to submit answer: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(team_answer))
}

// POST /quiz/instance/{instanceId}/state - Update instance state (future use)
pub async fn update_instance_state(
    State(state): State<AppState>,
    Path(instance_id): Path<String>,
    Json(payload): Json<UpdateInstanceState>,
) -> Result<StatusCode, StatusCode> {
    let uuid = uuid::Uuid::parse_str(&instance_id)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Verify instance exists
    let instance_exists = sqlx::query("SELECT id FROM quiz_instance WHERE uuid = $1")
        .bind(uuid)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| {
            tracing::error!("Failed to check instance: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
        .is_some();

    if !instance_exists {
        return Err(StatusCode::NOT_FOUND);
    }

    // For now, just acknowledge the state update
    // You can add a state column to quiz_instance table if needed
    tracing::info!("State update for instance {}: {:?}", uuid, payload.state);

    Ok(StatusCode::OK)
}
