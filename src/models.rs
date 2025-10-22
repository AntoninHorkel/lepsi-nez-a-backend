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

// Answer structure for JSONB field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub key: String,
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    pub content: String,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuestion {
    pub quiz_id: i32,
    pub q: String,
    pub answers: Vec<Answer>,
}

// QuizInstance model
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct QuizInstance {
    pub id: i32,
    pub uuid: Uuid,
    pub quiz_id: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateQuizInstance {
    pub quiz_id: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTeamAnswer {
    pub num: i32,
    pub ans: String,
    pub question_id: i32,
    pub quiz_instance_id: i32,
}
