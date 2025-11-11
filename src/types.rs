use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Quiz {
    pub name: String,
    pub questions: Vec<Question>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub text: String,
    pub answers: Vec<Answer>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Answer {
    pub text: String,
    pub isCorrect: bool,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct QuizSQL {
    pub id: Uuid,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct QuestionSQL {
    pub id: Uuid,
    pub quiz_id: Uuid,
    pub text: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct AnswerSQL {
    pub id: Uuid,
    pub question_id: Uuid,
    pub text: String,
    pub is_correct: bool,
}
