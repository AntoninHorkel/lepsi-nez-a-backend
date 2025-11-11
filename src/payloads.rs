use serde::Deserialize;
use uuid::Uuid;

use crate::types::{Quiz, QuizInstanceState};

pub type CreateQuizPayload = Quiz;

#[derive(Debug, Deserialize)]
pub struct CreateInstancePayload {}

#[derive(Debug, Deserialize)]
pub struct UpdateInstanceStatePayload {
    pub id: Uuid,
    pub state: QuizInstanceState,
}

#[derive(Debug, Deserialize)]
pub struct PostAnswerPayload {}
