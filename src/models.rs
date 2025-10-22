use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;

// Quiz model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Quiz {
    pub id: i32,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuiz {
    pub name: String,
}

// Question model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Question {
    pub id: i32,
    pub quiz_id: i32,
    pub q: String,
    pub answers: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// QuizInstance model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuizInstance {
    pub id: i32,
    pub uuid: Uuid,
    pub quiz_id: i32,
    pub created_at: DateTime<Utc>,
}

// TeamAnswer model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamAnswer {
    pub id: i32,
    pub num: i32,
    pub ans: String,
    pub question_id: i32,
    pub quiz_instance_id: i32,
    pub created_at: DateTime<Utc>,
}

// Additional types for API responses

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizWithQuestions {
    pub quiz: Quiz,
    pub questions: Vec<Question>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizInstanceDetail {
    pub instance: QuizInstance,
    pub quiz: Quiz,
    pub questions: Vec<Question>,
    pub team_answers: Vec<TeamAnswer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAnswer {
    pub team_id: i32,
    pub question_id: i32,
    pub ans: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInstanceState {
    pub state: String,
}
